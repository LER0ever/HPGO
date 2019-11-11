#ifndef GRAPH_H
#define GRAPH_H

#include <boost/python.hpp>
#include <string>
#include <vector>
#include <HPGO/HPGO_api.h>

namespace py = boost::python;
using pyo    = py::object;

class HPGO_API Graph {
 public:
  Graph();
  Graph(std::string filename);

  // C++ Members
  std::vector<std::vector<double>> compute_times;
  std::vector<std::vector<double>> activation_sizes;
  std::vector<std::vector<double>> parameter_sizes;
  std::vector<std::vector<int>>    all_predecessor_ids;
  std::vector<double>              output_activation_sizes;

  pyo  getStates();
  pyo  getGR();
  void update_stage_id(int end, int stage_id);

 private:
  // Python members
  pyo mUtils;
  pyo py_states;
  pyo py_gr;
  pyo py_compute_times;
  pyo py_activation_sizes;
  pyo py_parameter_sizes;
  pyo py_output_activation_sizes;
  pyo py_all_predecessor_ids;
};

// py::object mGraph = py::import("graph");
// py::object iGraph = mGraph.attr("Graph")();

#endif