#ifndef HPGO_H
#define HPGO_H

#include <vector>
#include <tuple>

using triple = std::tuple<double, std::pair<int, int>, double>;
using TypeA = std::vector<std::vector<std::vector<triple> > >;

TypeA compute_partitioning(
        std::vector<double> compute_times,
        std::vector<double> activation_sizes,
        std::vector<double> parameter_sizes,
        std::vector<double> output_activation_sizes,
        std::vector<int> all_predecessor_ids,
        int num_machines,
        int num_machines_within_machine,
        std::vector<double> bandwidth,
        int final_level
                );

#endif // HPGO_H