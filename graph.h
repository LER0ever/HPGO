#ifndef GRAPH_H
#define GRAPH_H

#include <string>

class Graph {};

class Node {
 public:
  int node_id;
  std::string node_desc;
  double fwd_comp_time;
  double bwd_comp_time;
  double activation_size;
  double parameter_size;
  int stage_id;
  int depth;
  int height;


};

#endif