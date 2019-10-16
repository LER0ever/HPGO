#ifndef ORCHESTRATION_H
#define ORCHESTRATION_H

#include <vector>
#include "duration.h"
#include "graph.h"
#include "model.h"
#include "syncpipeline.h"

using triple = std::tuple<double, std::pair<int, int>, int>;
using TypeA  = std::vector<std::vector<std::vector<triple> > >;
using d2d    = std::vector<std::vector<double> >;
using i2d    = std::vector<std::vector<int> >;

class Conductor {
 public:
  void setModel(Model);
  void orchestrate(std::vector<int> all_num_machines, std::vector<double> all_bandwidths,
                   std::string profile_filename);
  void orchestrate_single(std::vector<int> all_num_machines, std::vector<double> all_bandwidths,
                   std::string profile_filename, int replica, double dp_bandwidth);

  TypeA PD_compute_partitioning(d2d compute_times, d2d activation_sizes, d2d parameter_sizes,
                                std::vector<double> output_activation_sizes,
                                i2d all_predecessor_ids, int num_machines,
                                int num_machines_within_machine, double bandwidth,
                                bool final_level);
  TypeA compute_partitioning(d2d compute_times, d2d activation_sizes, d2d parameter_sizes,
                             std::vector<double> output_activation_sizes, i2d all_predecessor_ids,
                             int num_machines, int num_machines_within_machine, double bandwidth,
                             bool final_level);
  std::vector<int> PD_analyse_partititioning(TypeA A, int start, int end, double network_bandwidth,
                                             int num_machines);
  std::vector<int> analyse_partititioning(TypeA A, int start, int end, double network_bandwidth,
                                          int num_machines);

  // Debug Helper
  void printA(TypeA& A);

 private:
  Model m;
  TypeA A;
};

#endif  // ORCHESTRATION_H
