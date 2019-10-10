#ifndef ORCHESTRATION_H
#define ORCHESTRATION_H

#include <vector>
#include "duration.h"
#include "model.h"
#include "syncpipeline.h"

using triple = std::tuple<double, std::pair<int, int>, int>;
using TypeA  = std::vector<std::vector<std::vector<triple> > >;
using d2d    = std::vector<std::vector<double> >;
using i2d    = std::vector<std::vector<int> >;

class Conductor {
 public:
  void setModel(Model);
  void orchestrate();

  // Debug Helper
  void printA(TypeA& A);

 private:
  TypeA            compute_partitioning(d2d compute_times, d2d activation_sizes,
                                        d2d parameter_sizes, d2d output_activation_sizes,
                                        i2d all_predecessor_ids, int num_machines,
                                        int num_machines_within_machine, double bandwidth,
                                        bool final_level);
  std::vector<int> analyse_partititioning();
  Model            m;
};

#endif  // ORCHESTRATION_H
