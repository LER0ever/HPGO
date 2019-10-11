# Copyright (c) Microsoft Corporation.
# Licensed under the MIT license.
# Copyleft (*) LER0ever

from __future__ import print_function

import argparse
from collections import OrderedDict
import csv
import math
import os

import sys
sys.path.append("..")
import graph
import utils

# DEBUG SWITCH
HPGO_DEBUG = True

def main(all_num_machines, profile_filename, network_bandwidths, memory_size,
         straight_pipeline, use_memory_constraint, use_fewer_machines,
         activation_compression_ratio, output_directory,
         print_configuration=True, verbose=False):
    gr = graph.Graph.from_str(open(profile_filename, 'r').read())

    # Zero out all metadata associated with inputs in graph, since the optimizer
    # shouldn't really get a choice with where to place the input (should always
    # be in the first stage).
    sources = gr.sources()
    nodes_to_remove = OrderedDict()
    for source in sources:
        if source.node_desc.startswith("Input"):
            source.forward_compute_time = 0.0
            source.backward_compute_time = 0.0
            source.activation_size = 0.0
            source.parameter_size = 0.0
            nodes_to_remove[source] = []
            for out_node in gr.edges[source.node_id]:
                nodes_to_remove[source].append(out_node)
            gr.remove_node(source)

    # Remove all unneeded sinks that are not used, makes code generation and
    # optimization easier.
    sinks = gr.sinks()
    for sink in sinks:
        if sink.node_desc.startswith("__getitem__"):
            gr.remove_node(sink)

    antichain_gr = gr.antichain_dag()
    states = antichain_gr.topological_sort()
    if verbose:
        print("Total number of states: %d" % len(states))
    states_indices = {}
    for i in range(len(states)):
        states_indices[states[i]] = i
    for i in range(len(states)):
        for antichain_node in states[i].antichain:
            states[i].output_activation_size += gr.nodes[antichain_node].activation_size

    for i in range(len(states)):
        antichain = states[i].antichain
        all_predecessors = gr.all_predecessors(antichain)
        states[i].compute_time = 0.0
        states[i].activation_size = 0.0
        states[i].parameter_size = 0.0
        for predecessor in all_predecessors:
            states[i].compute_time += ((predecessor.forward_compute_time +
                                        predecessor.backward_compute_time) / 1000.0)
            states[i].activation_size += predecessor.activation_size
            states[i].parameter_size += predecessor.parameter_size
    gr.reset()

    output_activation_sizes = [state.output_activation_size for state in states]
    all_predecessor_ids = [[states_indices[predecessor] for predecessor in
                            antichain_gr.predecessors(states[i].node_id)]
                           for i in range(len(states))]

    compute_times = []
    activation_sizes = []
    parameter_sizes = []
    for i in range(len(states)+1):
        compute_times_row = []
        activation_sizes_row = []
        parameter_sizes_row = []
        for j in range(len(states)):
            if i == 0:
                compute_times_row.append(states[j].compute_time)
                activation_sizes_row.append(states[j].activation_size)
                parameter_sizes_row.append(states[j].parameter_size)
            else:
                if j > (i-1):
                    compute_times_row.append(states[j].compute_time -
                        states[i-1].compute_time)
                    activation_sizes_row.append(states[j].activation_size -
                        states[i-1].activation_size)
                    parameter_sizes_row.append(states[j].parameter_size -
                        states[i-1].parameter_size)
                else:
                    compute_times_row.append(None)
                    activation_sizes_row.append(None)
                    parameter_sizes_row.append(None)
        compute_times.append(compute_times_row)
        activation_sizes.append(activation_sizes_row)
        parameter_sizes.append(parameter_sizes_row)

    counter = 1
    all_As = []
    num_machines_in_machine = 1
    for num_machines, network_bandwidth in zip(all_num_machines, network_bandwidths):
        print("Solving optimization problem with %d machines with inter-machine bandwidth of %.2f GB/s" % (num_machines, network_bandwidth / 10**9))
        import numpy as np
        print(np.array(compute_times))
        if HPGO_DEBUG:
            print("compute_times", compute_times)
            print("activation_sizes", activation_sizes)
            print("parameter_sizes", parameter_sizes)
            print("output_activation_sizes", output_activation_sizes)
            print("all_predecessor_ids", all_predecessor_ids)
            print("num_machines", num_machines)
            print("num_machines_in_machine", num_machines_in_machine)
            print("network_bandwidth", network_bandwidth)
            print("final_level", (counter==len(network_bandwidths)))
        A = compute_partitioning(compute_times, activation_sizes, parameter_sizes,
                                 output_activation_sizes, all_predecessor_ids,
                                 num_machines, num_machines_in_machine,
                                 network_bandwidth,
                                 final_level=(counter==len(network_bandwidths)))
        num_machines_in_machine = num_machines
        for i in range(len(compute_times)):
            for j in range(len(compute_times[0])):
                compute_times[i][j] = A[i][j][-1][0]
        counter += 1
        all_As.append(A)
    print(np.array(compute_times))

    splits = [(0, len(states))]
    i = len(all_As) - 1
    while i >= 0:
        print("======================================")
        print("Level %d" % (i+1))
        print("======================================")
        new_splits = []
        stage_id = 0
        for (start, end) in splits:
            partial_splits = \
                analyze_partitioning(all_As[i], states, start, end,
                                     network_bandwidths[i], all_num_machines[i],
                                     activation_compression_ratio,
                                     print_configuration, verbose)
            start_point = start
            for split in partial_splits:
                new_splits.append((start_point, split))
                if i == 0:
                    predecessors = gr.all_predecessors(states[split-1].antichain)
                    for predecessor in predecessors:
                        if predecessor.stage_id is None:
                            predecessor.set_stage_id(stage_id)
                start_point = split
                stage_id += 1
            new_splits.append((start_point, end))
            if i == 0:
                predecessors = gr.all_predecessors(states[end-1].antichain)
                for predecessor in predecessors:
                    if predecessor.stage_id is None:
                        predecessor.set_stage_id(stage_id)
            stage_id += 1
        print("Total number of stages: %d" % stage_id)
        splits = new_splits
        i -= 1

    for source in nodes_to_remove:
        for out_node in nodes_to_remove[source]:
            source.stage_id = 0
            gr.add_edge(source, out_node)

    if output_directory is not None:
        total_num_machines = 1
        for num_machines in all_num_machines:
            total_num_machines *= num_machines
        gr.to_dot(os.path.join(output_directory, "gpus=%d" % total_num_machines))
        gr_str = str(gr)
        with open(os.path.join(output_directory, "gpus=%d.txt" % total_num_machines), 'w') as f:
            f.write(gr_str)

    total_time = states[-1].compute_time
    total_parameter_size = states[-1].parameter_size
    data_parallel_total_time = total_time
    num_machines_in_machine = 1
    for (num_machines, network_bandwidth) in zip(all_num_machines, network_bandwidths):
        data_parallel_communication_time = (
            (4 * (num_machines - 1) * total_parameter_size) /
            (network_bandwidth * num_machines)) / num_machines_in_machine
        data_parallel_total_time = sum(
            [data_parallel_total_time, data_parallel_communication_time]) / num_machines
        num_machines_in_machine = num_machines
    pipeline_parallel_total_time = A[0][len(states)-1][num_machines-1][0]

    if verbose:
        print()
        print("Time taken by single-stage pipeline:", total_time)
        print("Time per stage in pipeline:", pipeline_parallel_total_time)
        print("Throughput increase (compared to single machine):",
              total_time / pipeline_parallel_total_time)
        dp_str = ",".join([str(elem) for elem in all_num_machines])
        print(("[Note that single-machine and (%s)-machine DP might not fit "
               "given memory constraints]") % dp_str)
        print("Throughput increase of (%s)-machine DP compared to single "
              "machine:" % dp_str, total_time / data_parallel_total_time)
        print("Throughput increase (compared to (%s)-machine DP):" % dp_str,
              data_parallel_total_time / pipeline_parallel_total_time)
    return pipeline_parallel_total_time, data_parallel_total_time


if __name__ == '__main__':
    parser = argparse.ArgumentParser(
        description=("Run PipeDream's optimizer for replicated settings")
    )
    parser.add_argument('-n', "--all_num_machines", nargs='+', type=int,
                        help="Number of machines available")
    parser.add_argument('-f', "--profile_filename", required=True,
                        help="Profile filename")
    parser.add_argument('-b', "--network_bandwidths", type=float, nargs='+', default=[1000000000],
                        help="Available network bandwidth in bytes/sec")
    parser.add_argument('-s', "--memory_size", type=float, default=16000000000,
                        help="Amount of memory available on each machine")
    parser.add_argument("--straight_pipeline", action='store_true',
                        help="No replication across stages")
    parser.add_argument('-o', "--output_directory", default=None, type=str,
                        help="Output directory to dump processed graph")
    parser.add_argument("--use_memory_constraint", action='store_true',
                        help="Enforce memory constraint per machine")
    parser.add_argument("--use_fewer_machines", action='store_true',
                        help="Use fewer machines, if possible")
    parser.add_argument("--activation_compression_ratio", default=None, type=float,
                        help="Compression ratio for activations")

    args = parser.parse_args()
    args = vars(args)

    all_num_machines = args["all_num_machines"]
    profile_filename = args["profile_filename"]
    network_bandwidths = args["network_bandwidths"]
    assert(len(all_num_machines) == len(network_bandwidths))
    memory_size = args["memory_size"]
    straight_pipeline = args["straight_pipeline"]
    output_directory = args["output_directory"]
    use_memory_constraint = args["use_memory_constraint"]
    use_fewer_machines = args["use_fewer_machines"]
    activation_compression_ratio = args["activation_compression_ratio"]

    main(all_num_machines, profile_filename, network_bandwidths, memory_size,
         straight_pipeline, use_memory_constraint, use_fewer_machines,
         activation_compression_ratio, output_directory,
         verbose=True)
