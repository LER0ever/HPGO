#include <HPGO/environment/device.h>
#include <HPGO/input/graph.h>
#include <HPGO/orchestration/orchestration.h>
#include <HPGO/utils/helper.h>
#include <HPGO/parallelism/data-parallel/data-parallel.h>
#include <HPGO/parallelism/split-concat/split-concat.h>

#include <algorithm>
#include <cassert>
#include <cmath>
#include <iostream>

using namespace std;

const double GIGA       = 1000000000;
const double B_ETHERNET = 1.2 * GIGA;
const double B_NVLINK   = 25 * GIGA;

#define DEBUG

void Conductor::orchestrate(int num_machines, std::vector<int> seps) {
  // NOTE: Disabled by Copybara
}

TA Conductor::compute_partitioning(d2d compute_times, d2d activation_sizes, d2d parameter_sizes,
                                   vector<double> output_activation_sizes, i2d all_predecessor_ids,
                                   int num_machines) {
  TA A;

  // Initialization
  fri(rp, 1, num_machines + 1) {
    vector<vector<unordered_map<ll, EA>>> row_A;
    frs(i, 0, compute_times[0].size()) {
      vector<unordered_map<ll, EA>> row_row_A;
      for (int j = 0; j < num_machines; j++) {
        unordered_map<ll, EA> um;
        row_row_A.push_back(um);
      }
      row_A.push_back(row_row_A);
    }
    A.push_back(row_A);
  }
  cout << "Initialization Done" << endl;

  // NOTE: temporary placeholder
  ll ph, empty;
  for (int i = 0; i < num_machines + 1; i++) ph.push_back(true);
  for (int i = 0; i < num_machines; i++) empty.push_back(false);

  // Devices d = Devices(num_machines, std::vector<int>{8, 16});
  Devices d  = Devices(num_machines, std::vector<int>{num_machines / 2, num_machines});
  int     rp = 1;

  // DP Initialization
  frs(j, 0, compute_times[0].size()) {
    double cur_compute_time    = compute_times[0][j];
    double cur_activation_size = activation_sizes[0][j];
    double cur_parameter_size  = parameter_sizes[0][j];
    int    max_m               = num_machines;  // TODO: check straight_pipeline
    fri(m, 0, max_m) {
      // auto bs_after_m = std::get<1>(d.bitnext(empty, m + 1)[0]);
      auto [n_switch, n_bitset, n_wids] = d.bitnext(empty, m + 1)[0];
      if (cur_compute_time < -0.5) {  // normally -1
        A[rp][j][m][ph]       = make_tuple(-1, make_pair(-1, -1), -1, set<int>{});
        A[rp][j][m][n_bitset] = make_tuple(-1, make_pair(-1, -1), -1, n_wids);
      } else {
        #ifdef DEBUG
        
        #endif
        A[rp][j][m][ph] = make_tuple(
            std::max((cur_compute_time) / (m + 1), 0.0 /*cur_parameter_size * 1 / B_ETHERNET*/) + DataParallel(d, n_wids, cur_parameter_size),
            make_pair(-1, -1), m + 1, set<int>{});
        A[rp][j][m][n_bitset] =
          make_tuple(std::max((cur_compute_time) / (m + 1), 0.0) + DataParallel(d, n_wids, cur_parameter_size), make_pair(-1, -1), m + 1, n_wids);

        // try to init -1 for everything
        // A[rp][j][m][ph]       = make_tuple(-1, make_pair(-1, -1), -1, set<int>{});
        // A[rp][j][m][n_bitset] = make_tuple(-1, make_pair(-1, -1), -1, n_wids);
      }
    }
  }

  cout << "DP Assignment Done" << endl;

  int min_m = 1;
  fri(m, min_m, num_machines) {  // NOTE: iterate over m available machines
    frs(j, 1, compute_times[0].size()) {
      double         min_pipeline_time;
      pair<int, int> optimal_split;
      int            optimal_num_machines;
      ll             optimal_bitset;
      set<int>       last_machines;
      if (A[rp][j][m].find(ph) != A[rp][j][m].end()) {
        min_pipeline_time    = get<0>(A[rp][j][m][ph]);
        optimal_split        = get<1>(A[rp][j][m][ph]);
        optimal_num_machines = get<2>(A[rp][j][m][ph]);
        last_machines        = get<3>(A[rp][j][m][ph]);
      } else {
        continue;
      }

      for (auto k : all_predecessor_ids[j]) {  // NOTE: iterate over j's predecessors
        int max_mp = m + 1;                    // TODO: straight pipeline check
        for (auto mp = 1; mp < max_mp; mp++) {
          for (auto& bs : A[rp][k][m - mp]) {
            if (bs.first.size() > num_machines) continue;  // skip ph

#ifdef DEBUG
            // for the current bs, print the bitset
            cout << "For BS: ";
            for (auto b : bs.first) cout << b;
            cout << endl;
#endif

            auto prev_bs     = bs.first;
            auto next_bs_all = d.bitnext(prev_bs, mp);
#ifdef DEBUG
            // print all the next available bs machines array
            for (auto s : next_bs_all) {
              cout << get<0>(s) << ": ";
              for (auto n : get<2>(s)) cout << n << " ";
            }
            cout << endl;
#endif

            for (auto nbs : next_bs_all) {
              auto [n_switch, n_bitset, n_wids] = nbs;  // NOTE: [FF, SF, AF], v<b>, v<i>
              double input_transfer_time        = (2.0 * output_activation_sizes[k]) /
                                           (B_ETHERNET * mp);  // TODO: hardcoded for debugging
              double output_transfer_time = -1;
              if (j < output_activation_sizes.size() - 1) {
                output_transfer_time = (2.0 * output_activation_sizes[j]) /
                                       (B_ETHERNET * mp);  // TODO: hardcoded for debugging
              }

              double last_stage_time = compute_times[k + 1][j];
              if (last_stage_time < -0.5) continue;  // = -1
              // double last_stage_parameter_size = parameter_sizes[k + 1][j];
              // last_stage_time += (4 * (mp - 1) * last_stage_parameter_size) / (bandwidth * mp);
              last_stage_time /= mp;

              if (A[rp][k][m - mp].find(ph) == A[rp][k][m - mp].end() ||
                  get<0>(A[rp][k][m - mp][ph]) < -0.5)
                continue;

              double pipeline_time = max(get<0>(A[rp][k][m - mp][ph]), last_stage_time);
              pipeline_time        = max(pipeline_time, input_transfer_time);
              if (output_transfer_time > -0.5)
                pipeline_time = max(pipeline_time, output_transfer_time);

              if (min_pipeline_time < -0.5 || min_pipeline_time > pipeline_time) {
                optimal_split        = make_pair(k, m - mp);
                optimal_num_machines = mp;
                min_pipeline_time    = pipeline_time;
#ifdef DEBUG
                cout <<"new min_pipeline_time: " << min_pipeline_time << endl;
#endif
              }

              A[rp][j][m][n_bitset] =
                  make_tuple(min_pipeline_time, optimal_split, optimal_num_machines, n_wids);
            }
          }
        }
      }
      A[rp][j][m][ph] =
          make_tuple(min_pipeline_time, optimal_split, optimal_num_machines, set<int>{});
    }
  }
  return A;
}

std::pair<std::vector<int>, std::vector<int>> Conductor::analyse_partititioning(TA A, int end,
                                                                                int num_machines) {
  ll ph;
  for (int i = 0; i < num_machines + 1; i++) ph.push_back(true);

  int rp = 1;  // NOTE: Hardcoded
  std::cout << "\033[33mEnter HPGO Analyse Partitioning\033[0m" << std::endl;
  auto             metadata                = A[rp][end - 1][num_machines - 1][ph];
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
    metadata   = A[rp][next_split.first][next_split.second][ph];
    next_split = std::get<1>(metadata);
    replication_factors.push_back(num_machines_used);
    remaining_machines_left -= num_machines_used;
  }

  int num_machines_used = std::get<2>(metadata);
  remaining_machines_left -= num_machines_used;

  //  std::cout << "\033[33m-------------------------------------\033[0m" << std::endl;
  //  std::cout << "\033[33mnum_machines_used: " << num_machines_used << "\033[0m" << std::endl;

  prev_split = 0;
  std::reverse(splits.begin(), splits.end());
  splits.push_back(end);
  replication_factors.push_back(num_machines_used);
  std::reverse(replication_factors.begin(), replication_factors.end());

  // std::cout << "\033[33m====================================\033[0m" << std::endl;
  // for (int i = 0; i < splits.size(); i++) {
  //   std::cout << "\033[33m (" << (i == 0 ? prev_split : splits[i - 1] + 1) << " ~ " << splits[i]
  //             << ") x " << replication_factors[i] << "\033[0m" << std::endl;
  // }
  // std::cout << "\033[33m====================================\033[0m" << std::endl;
  //  std::cout << splits.size() << replication_factors.size() << std::endl;

  pair<vector<int>, vector<int>> res;
  res.first  = splits;
  res.second = replication_factors;
  return res;
}

void Conductor::printA(TA& A) {
  frs(rp, 0, A.size()) {
    // cout << "rp = " << rp << endl;
    frs(j, 0, A[rp].size()) {
      // cout << "    j = " << j << endl;
      frs(m, 0, A[rp][j].size()) {
        // cout << "    m = " << m << endl;
        for (auto& bs : A[rp][j][m]) {
          cout << "A[" << rp << "][" << j << "][" << m << "][?] = \t";
          for (auto b : bs.first) {
            cout << b;
          }
          cout << "\t :: \t";
          auto [min_pipeline_time, optimal_split, optimal_num_machines, wids] = bs.second;
          cout << "[" << min_pipeline_time << ", \t(" << optimal_split.first << ", "
               << optimal_split.second << "), \t" << optimal_num_machines << ", \t(";
          for (auto s : wids) {
            cout << s << " ";
          }
          cout << ")]" << endl;
        }
      }
    }
  }
}

void Conductor::setProfileFilename(std::string fn) {
  this->profile_filename = fn;
  Graph g                = Graph(fn);
  this->setGraph(g);
  Model m = Model(1024, 128, g);  // FIXME: hardcoded gbs and pbs
  // m.Normalize();
  // m.SetLayerStats(g.compute_times, g.activation_sizes, g.parameter_sizes,
  //                 g.output_activation_sizes);
  this->setModel(m);  // TODO: interface for global batch size
}
void Conductor::setModel(Model m) { this->m = m; }
void Conductor::setGraph(Graph g) { this->g = g; }
void Conductor::setDevices(Devices d) { this->d = d; }
