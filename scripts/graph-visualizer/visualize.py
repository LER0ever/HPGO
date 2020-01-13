
import csv
import os

import sys
sys.path.append("../../contrib")
import graph

def convert_graph(graph_filename, output_directory, arch):
    print("Processing %s..." % graph_filename)
    gr = graph.Graph.from_str(open(graph_filename, 'r').read())
    flattened_gr = gr.flattened_graph()
    gr.check_fidelity(flattened_gr)
    output_directory = os.path.join(output_directory, arch)
    flattened_gr.to_dot(os.path.join(output_directory, "graph"))
    with open(os.path.join(output_directory, "graph.txt"), 'w') as f:
        f.write(str(flattened_gr))

graph_filename = "../../profiles/amoebanet_36/graph.txt"

with open(graph_filename, 'r') as f:
    graph_str = f.read()
gr = graph.Graph.from_str(graph_str)

gr.to_dot("./amoebanet_36.dot")

print(gr.flattened_graph())

for i in gr.topological_sort():
    print(i)

convert_graph(graph_filename, "./flattened", "amoebanet_36")
