from collections import OrderedDict

import sys

sys.path.append("..")
import graph
import utils


def prepare(profile_filename, verbose=False):
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
    for i in range(len(states) + 1):
        compute_times_row = []
        activation_sizes_row = []
        parameter_sizes_row = []
        for j in range(len(states)):
            if i == 0:
                compute_times_row.append(states[j].compute_time)
                activation_sizes_row.append(states[j].activation_size)
                parameter_sizes_row.append(states[j].parameter_size)
            else:
                if j > (i - 1):
                    compute_times_row.append(states[j].compute_time -
                                             states[i - 1].compute_time)
                    activation_sizes_row.append(states[j].activation_size -
                                                states[i - 1].activation_size)
                    parameter_sizes_row.append(states[j].parameter_size -
                                               states[i - 1].parameter_size)
                else:
                    compute_times_row.append(-1.0)
                    activation_sizes_row.append(-1.0)
                    parameter_sizes_row.append(-1.0)
        compute_times.append(compute_times_row)
        activation_sizes.append(activation_sizes_row)
        parameter_sizes.append(parameter_sizes_row)
    # for i in range(len(states)):
    #     print(i, compute_times[i][i])
    #     this would give you the layer-wise compute time
    return gr, states, compute_times, activation_sizes, parameter_sizes, output_activation_sizes, all_predecessor_ids


def update_stage_id(gr, states, end, stage_id):
    predecessors = gr.all_predecessors(states[end-1].antichain)
    for predecessor in predecessors:
        if predecessor.stage_id is None:
            predecessor.set_stage_id(stage_id)