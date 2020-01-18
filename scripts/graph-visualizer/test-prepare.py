



import csv
import os

import sys
sys.path.append("../../contrib")
import torch_graph

torch_graph.prepare("../../profiles/gnmt_32/graph.txt", True)