#include "orchestration.h"

#include <algorithm>
#include <iostream>

void Conductor::setModel(Model m) { this->m = m; }

TypeA Conductor::PD_compute_partitioning(d2d compute_times, d2d activation_sizes,
                                         d2d                 parameter_sizes,
                                         std::vector<double> output_activation_sizes,
                                         i2d all_predecessor_ids, int num_machines,
                                         int num_machines_within_machine, double bandwidth,
                                         bool final_level) {
  TypeA A;

  // Initialization
  for (int i = 0; i < compute_times.size(); i++) {
    std::vector<std::vector<triple>> row_A;
    for (int j = 0; j < compute_times[i].size(); j++) {
      std::vector<triple> row_row_A;
      for (int m = 0; m < num_machines; m++)
        row_row_A.push_back(std::make_tuple(-1, std::make_pair(-1, -1), -1));
      row_A.push_back(row_row_A);
    }
    A.push_back(row_A);
  }

  // DBG
  for (auto vc : compute_times) { std::cout << "["; for (auto c :vc) { std::cout << c << ",";} std::cout << "]" << std::endl;}

  for (auto i = 0; i < compute_times.size(); i++) {
    for (auto j = i; j < compute_times[0].size(); j++) {
      double cur_compute_time    = compute_times[i][j];
      double cur_activation_size = activation_sizes[i][j];
      double cur_parameter_size  = parameter_sizes[i][j];
      int    max_m               = num_machines;  // TODO: check straight_pipeline
      for (int m = 0; m < max_m; m++) {
        // TODO: Memory constraint check
        double dp_comm_time =
            (4.0 * m * cur_parameter_size) / (bandwidth * (m + 1));  // TODO: Bandwidth curve
        dp_comm_time /= num_machines_within_machine;
        if (cur_compute_time < -0.5)  // normally -1
          A[i][j][m] = std::make_tuple(-1, std::make_pair(-1, -1), -1);
        else
          A[i][j][m] = std::make_tuple((cur_compute_time + dp_comm_time) / (m + 1),
                                       std::make_pair(-1, -1), m + 1);
      }
    }
  }

  // TODO: zero-overhead backtracking
  int min_m = 1;
  int max_i = final_level ? 1 : compute_times.size();

  for (auto i = 0; i < max_i; i++) {
    for (auto m = min_m; m < num_machines; m++) {
      for (auto j = i + 1; j < compute_times[0].size(); j++) {
        auto [min_pipeline_time, optimal_split, optimal_num_machines] = A[i][j][m];
        // TODO: use fewer machines check
        for (auto k : all_predecessor_ids[j]) {
          std::cout << "DBG: K = " << k << std::endl;
          if (i > 0 &&
              std::find(all_predecessor_ids[i - 1].begin(), all_predecessor_ids[i - 1].end(), k) !=
                  all_predecessor_ids[i - 1].end())
            continue;
          int max_mp = m + 1;  // TODO: straight pipeline check
          for (auto mp = 1; mp < max_mp; mp++) {
            double input_transfer_time  = (2.0 * output_activation_sizes[k]) / (bandwidth * mp);
            double output_transfer_time = -1;
            if (j < output_activation_sizes.size() - 1) {
              output_transfer_time = (2.0 * output_activation_sizes[j]) / (bandwidth * mp);
            }

            double last_stage_time = compute_times[k + 1][j];
            if (last_stage_time < -0.5) {std::cout << "Passed due to last_stage_time " << k+1 << "," << j <<std::endl;continue; } // = -1
            double last_stage_parameter_size = parameter_sizes[k + 1][j];
            // TODO: Memory check, stashed data
            last_stage_time += (4 * (mp - 1) * last_stage_parameter_size) / (bandwidth * mp);
            last_stage_time /= mp;

            if (std::get<0>(A[i][k][m - mp]) < -0.5) {std::cout << "Passed due to A[i][k][m - mp][0]" <<std::endl;   continue;}

            double pipeline_time = std::max(std::get<0>(A[i][k][m - mp]), last_stage_time);
            if (min_pipeline_time < -0.5 || min_pipeline_time > pipeline_time) {
              std::cout << "enter final if, K = " << k << std::endl;
              optimal_split        = std::make_pair(k, m - mp);
              optimal_num_machines = mp;
              min_pipeline_time    = pipeline_time;
            }
          }
        }
        A[i][j][m] = std::make_tuple(min_pipeline_time, optimal_split, optimal_num_machines);
      }
    }
  }

  // Assign to class var and return
  this->A = A;
  return A;
}

std::vector<int> Conductor::analyse_partititioning(TypeA A, int start, int end,
                                                   double network_bandwidth, int num_machines) {
  std::cout << "\033[33mEnter Analyse Partitioning\033[0m" << std::endl;
  auto             metadata                = A[start][end - 1][num_machines - 1];
  auto             next_split              = std::get<1>(metadata);
  int              remaining_machines_left = num_machines;
  std::vector<int> splits;
  std::vector<int> replication_factors;
  int              prev_split = end - 1;

  std::cout << "\033[33m" << next_split.first << ", " << next_split.second << "\033[0m" << std::endl;

  while (next_split.first != -1) {  // -1 means None
    int num_machines_used = std::get<2>(metadata);

    // FIX: redundant print
    std::cout << "\033[33m-------------------------------------\033[0m" << std::endl;
    std::cout << "\033[33mnum_machines_used: " << num_machines_used << "\033[0m" << std::endl;
    std::cout << "\033[33msplit between: " << next_split.first << " and " << next_split.first + 1
              << "\033[0m" << std::endl;

    splits.push_back(next_split.first + 1);
    prev_split = splits[splits.size() - 1];
    metadata   = A[start][next_split.first][next_split.second];
    next_split = std::get<1>(metadata);
    replication_factors.push_back(num_machines_used);
    remaining_machines_left -= num_machines_used;
  }

  int num_machines_used = std::get<2>(metadata);
  remaining_machines_left -= num_machines_used;

  prev_split = start;
  std::reverse(splits.begin(), splits.end());
  splits.push_back(end);
  replication_factors.push_back(num_machines_used);
  std::reverse(replication_factors.begin(), replication_factors.end());

  std::cout << splits.size() << replication_factors.size() << std::endl;

  std::cout << "\033[33m====================================\033[0m" << std::endl;
  for (int i=0; i<splits.size(); i++) {
    std::cout << "\033[33m (" << (i == 0 ? 0 : splits[i-1] + 1) << " ~ " << splits[i]  << ") x " << replication_factors[i] << "\033[0m" << std::endl;
  }
  std::cout << "\033[33m====================================\033[0m" << std::endl;

  return splits;
}

void Conductor::printA(TypeA& A) {
  for (int i = 0; i < A.size(); i++) {
    for (int j = 0; j < A[i].size(); j++) {
      for (int m = 0; m < A[i][j].size(); m++) {
        auto [pipeline_time, opt_split, opt_num_machines] = A[i][j][m];
        std::cout << "A[" << i << "][" << j << "][" << m << "] = (" << pipeline_time << ", ("
                  << opt_split.first << ", " << opt_split.second << "), " << opt_num_machines
                  << std::endl;
      }
    }
  }
}

void Conductor::orchestrate() {}
