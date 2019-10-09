#include "orchestration.h"

#include <iostream>

void Conductor::setModel(Model m ) {
    this->m = m;
}

TypeA Conductor::compute_partitioning(d2d compute_times, d2d activation_sizes, d2d parameter_sizes,
                                      d2d output_activation_sizes, i2d all_predecessor_ids, int num_machines,
                                      int num_machines_within_machine, double bandwidth,
                                      bool final_level) {
    TypeA A;

    // Initialization
    for (int i=0; i<compute_times.size(); i++) {
        std::vector<std::vector<triple> > row_A;
        for (int j=0; j<compute_times[i].size(); j++) {
            std::vector<triple> row_row_A;
            for (int m=0; m<num_machines; m++)
                row_row_A.push_back(std::make_tuple(-1, std::make_pair(-1, -1), -1));
            row_A.push_back(row_row_A);
        }
        A.push_back(row_A);
    }

    for (int i=0; i<compute_times.size(); i++) {
        for (int j=0; j<compute_times[i].size(); j++) {
            double cur_compute_time = compute_times[i][j];
            double cur_activation_size = activation_sizes[i][j];
            double cur_parameter_size = parameter_sizes[i][j];
            int max_m = num_machines; // TODO: check straight_pipeline
            for (int m=0; m<max_m; m++) {
                // TODO: Memory constraint check
                double dp_comm_time = (2 * m * cur_parameter_size) / (bandwidth * (m+1)); // TODO: Bandwidth curve
                dp_comm_time /= num_machines_within_machine;
                if (cur_compute_time == -1)
                    A[i][j][m] = std::make_tuple(-1, std::make_pair(-1, -1), -1);
                else
                    A[i][j][m] = std::make_tuple((cur_compute_time + dp_comm_time) / (m+1), std::make_pair(-1, -1), m+1);
            }
        }
    }

    return A;
}

void Conductor::printA(TypeA &A) {
    for (int i=0; i< A.size(); i++) {
        for (int j=0; j<A[i].size(); j++) {
            for (int m=0; m<A[i][j].size(); m++) {
                auto [pipeline_time, opt_split, opt_num_machines] = A[i][j][m];
                std::cout << "A[" << i << "][" << j << "][" << m << "] = ("
                << pipeline_time << ", (" << opt_split.first << ", "
                << opt_split.second << "), " << opt_num_machines
                << std::endl;
            }
        }
    }
}

void Conductor::orchestrate() {

}