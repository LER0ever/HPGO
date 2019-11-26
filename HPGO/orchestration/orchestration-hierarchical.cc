#include <HPGO/input/graph.h>
#include <HPGO/orchestration/orchestration.h>

#include <algorithm>
#include <cassert>
#include <cmath>
#include <iostream>

void HierarchicalConductor::orchestrate(std::vector<int>    all_num_machines,
                                        std::vector<double> all_bandwidths) {
  assert(this->profile_filename != "");

  orchestrate_single(all_num_machines, all_bandwidths, 1, 0);
  // this is currently a hack
  // TODO: replace this with a full Max-Flow Min-Cut based algorithm
  assert(all_num_machines.size() == 2);
  int total_num_machines = 1;
  for (auto nm : all_num_machines) total_num_machines *= nm;
  for (int i = 2; i <= floor(total_num_machines / 2.0 + 0.5); i++) {
    std::vector<int>    new_num_machines;
    std::vector<double> new_bandwidths;
    if (i <= all_num_machines[0]) {
      new_bandwidths.push_back(all_bandwidths[1]);
      new_num_machines.push_back(i);
      std::cout << std::endl << "DPU = " << i << std::endl;
      orchestrate_single(new_num_machines, new_bandwidths, total_num_machines / i,
                         all_bandwidths[0]);
    } else {
      std::cout << "Apparently I did not handle this case :(" << std::endl;
    }
  }
}

void HierarchicalConductor::orchestrate_single(std::vector<int>    all_num_machines,
                                               std::vector<double> all_bandwidths, int replica,
                                               double dp_bandwidth) {
  assert(all_num_machines.size() == all_bandwidths.size());

  auto compute_times           = this->m.Meta.compute_times;
  auto activation_sizes        = this->m.Meta.activation_sizes;
  auto parameter_sizes         = this->m.Meta.parameter_sizes;
  auto output_activation_sizes = this->m.Meta.output_activation_sizes;
  auto all_predecessor_ids     = this->m.Meta.all_predecessor_ids;

  std::vector<TypeA> As;
  int                cnt                     = 1;
  int                num_machines_in_machine = 1;
  for (int e = 0; e < all_num_machines.size(); e++) {
    auto num_machines = all_num_machines[e];
    auto bandwidth    = all_bandwidths[e];
    std::cout << "Solving for " << num_machines << " machines with bandwidth "
              << bandwidth / 1000000000 << " GB/s" << std::endl;
    auto A =
        compute_partitioning(compute_times, activation_sizes, parameter_sizes,
                             output_activation_sizes, all_predecessor_ids, num_machines,
                             num_machines_in_machine, bandwidth, (cnt == all_bandwidths.size()));
    num_machines_in_machine = num_machines;
    for (int i = 0; i < compute_times.size(); i++)
      for (int j = 0; j < compute_times[0].size(); j++) {
        compute_times[i][j] = std::get<0>(A[i][j][A[i][j].size() - 1]);
      }
    cnt++;
    As.push_back(A);
  }

  std::vector<std::pair<int, int>> splits;
  int                              stage_id = 0;
  splits.push_back(std::make_pair(0, compute_times[0].size()));  // TODO: change to len(states)
  for (int i = As.size() - 1; i >= 0; i--) {
    std::cout << "Level " << i + 1 << std::endl;
    std::vector<std::pair<int, int>> new_splits;

    for (auto s : splits) {
      int start = s.first;
      int end   = s.second;
      std::cout << "Analyze Partitioning (" << start << ", " << end << ")" << std::endl;
      auto partial_splits =
          analyse_partititioning(As[i], start, end, all_bandwidths[i], all_num_machines[i]);
      //      std::cout << "Partial Splits: ";
      //      for (auto ps : partial_splits) std::cout << ps << " ";
      //      std::cout << std::endl;

      int start_point = start;
      for (auto ps : partial_splits) {
        new_splits.push_back(std::make_pair(start_point, ps));
        if (i == 0) {
          this->g.update_stage_id(ps, stage_id);
        }
        start_point = ps;
        stage_id += 1;
      }
      new_splits.push_back(std::make_pair(start_point, end));
      if (i == 0) {
        this->g.update_stage_id(end, stage_id);
      }
      stage_id += 1;
    }
    std::cout << "Total stage # = " << stage_id - 1 << std::endl;  // Fix Offset
    splits = new_splits;
  }

  // TODO: write graph

  // TODO: this is currently a rough estimation
  // Accurate modeling to be moved to model.cc, syncpipeline.cc, device.cc and block.cc
  pyo    states     = g.getStates();
  double total_time = py::extract<double>(states[py::len(states) - 1].attr("compute_time"));
  double total_parameter_size =
      py::extract<double>(states[py::len(states) - 1].attr("parameter_size"));
  double data_parallel_total_time = total_time;
  //  std::cout << "dp original total = " << data_parallel_total_time << std::endl;
  num_machines_in_machine = 1;
  for (int e = 0; e < all_num_machines.size(); e++) {
    auto   num_machines = all_num_machines[e];
    auto   bandwidth    = all_bandwidths[e];
    double data_parallel_communication_time;
    if (replica > 1) {
      data_parallel_communication_time = ((2 * (num_machines - 1) * total_parameter_size) /
                                          (dp_bandwidth * num_machines * (num_machines / 2))) /
                                         num_machines_in_machine;
    } else {
      data_parallel_communication_time = ((2 * (num_machines - 1) * total_parameter_size) /
                                          (bandwidth * num_machines /* * (num_machines / 2)*/)) /
                                         num_machines_in_machine;
    }

    data_parallel_total_time = (data_parallel_total_time + data_parallel_communication_time);
    num_machines_in_machine  = num_machines;
  }
  double pipeline_parallel_total_time =
      std::get<0>(A[0][len(states) - 1][all_num_machines[all_num_machines.size() - 1] - 1]);
  pipeline_parallel_total_time *= 2 * (stage_id - 1);

  if (replica > 1) {
    double pipeline_dp_communication_time =
        ((2 * (replica - 1) * total_parameter_size) / (dp_bandwidth * replica * (replica / 2))) /
        num_machines_in_machine;
    pipeline_parallel_total_time += pipeline_dp_communication_time;
  }

  std::cout << "This speedup calculation is currently NOT correct" << std::endl;
  std::cout << "Normalized HPGO: " << pipeline_parallel_total_time
            << " DP: " << data_parallel_total_time << std::endl
            << "HPGO Solution Speedup over DP: "
            << pipeline_parallel_total_time / data_parallel_total_time << std::endl;
}

TypeA HierarchicalConductor::compute_partitioning(d2d compute_times, d2d activation_sizes,
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
            if (last_stage_time < -0.5) continue;  // = -1
            double last_stage_parameter_size = parameter_sizes[k + 1][j];
            // TODO: Memory check, stashed data
            last_stage_time += (4 * (mp - 1) * last_stage_parameter_size) / (bandwidth * mp);
            last_stage_time /= mp;

            if (std::get<0>(A[i][k][m - mp]) < -0.5) continue;

            double pipeline_time = std::max(std::get<0>(A[i][k][m - mp]), last_stage_time);
            pipeline_time        = std::max(pipeline_time, input_transfer_time);
            if (output_transfer_time > -0.5)
              pipeline_time = std::max(pipeline_time, output_transfer_time);

            if (min_pipeline_time < -0.5 || min_pipeline_time > pipeline_time) {
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

std::vector<int> HierarchicalConductor::analyse_partititioning(TypeA A, int start, int end,
                                                               double network_bandwidth,
                                                               int    num_machines) {
  std::cout << "\033[33mEnter HPGO Analyse Partitioning\033[0m" << std::endl;
  auto             metadata                = A[start][end - 1][num_machines - 1];
  auto             next_split              = std::get<1>(metadata);
  int              remaining_machines_left = num_machines;
  std::vector<int> splits;
  std::vector<int> replication_factors;
  int              prev_split = end - 1;

  std::cout << "\033[33m" << next_split.first << ", " << next_split.second << "\033[0m"
            << std::endl;

  while (next_split.first != -1) {  // -1 means None
    int num_machines_used = std::get<2>(metadata);

    // FIX: redundant print
    //    std::cout << "\033[33m-------------------------------------\033[0m" << std::endl;
    //    std::cout << "\033[33mnum_machines_used: " << num_machines_used << "\033[0m" << std::endl;
    //    std::cout << "\033[33msplit between: " << next_split.first << " and " << next_split.first
    //    + 1
    //              << "\033[0m" << std::endl;

    splits.push_back(next_split.first + 1);
    prev_split = splits[splits.size() - 1];
    metadata   = A[start][next_split.first][next_split.second];
    next_split = std::get<1>(metadata);
    replication_factors.push_back(num_machines_used);
    remaining_machines_left -= num_machines_used;
  }

  int num_machines_used = std::get<2>(metadata);
  remaining_machines_left -= num_machines_used;

  //  std::cout << "\033[33m-------------------------------------\033[0m" << std::endl;
  //  std::cout << "\033[33mnum_machines_used: " << num_machines_used << "\033[0m" << std::endl;

  prev_split = start;
  std::reverse(splits.begin(), splits.end());
  splits.push_back(end);
  replication_factors.push_back(num_machines_used);
  std::reverse(replication_factors.begin(), replication_factors.end());

  //  std::cout << splits.size() << replication_factors.size() << std::endl;

  std::cout << "\033[33m====================================\033[0m" << std::endl;
  for (int i = 0; i < splits.size(); i++) {
    std::cout << "\033[33m (" << (i == 0 ? prev_split : splits[i - 1] + 1) << " ~ " << splits[i]
              << ") x " << replication_factors[i] << "\033[0m" << std::endl;
  }
  std::cout << "\033[33m====================================\033[0m" << std::endl;

  return splits;
}

void HierarchicalConductor::printA(TypeA& A) {
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

void HierarchicalConductor::setProfileFilename(std::string fn) {
  this->profile_filename = fn;
  Graph g                = Graph(fn);
  this->setGraph(g);
  Model m = Model(1024, 32, 32, g);
  this->setModel(m);  // TODO: interface for global batch size
}
void HierarchicalConductor::setModel(Model m) { this->m = m; }
void HierarchicalConductor::setGraph(Graph g) { this->g = g; }
