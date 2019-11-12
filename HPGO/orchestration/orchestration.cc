#include <HPGO/environment/device.h>
#include <HPGO/input/graph.h>
#include <HPGO/orchestration/orchestration.h>
#include <HPGO/parallelism/data-parallel/data-parallel.h>
#include <HPGO/parallelism/split-concat/split-concat.h>
#include <HPGO/utils/helper.h>

#include <algorithm>
#include <cassert>
#include <cmath>
#include <iostream>

// NOTE: consider removing std global scope
using namespace std;

// #define DEBUG

void Conductor::orchestrate() {
  ll  empty;
  int num_machines = this->d.G;
  for (int i = 0; i < num_machines; i++) empty.push_back(false);

  auto   mt                    = m.Meta;
  double total_compute_time    = mt.compute_times[0][mt.compute_times[0].size() - 1];
  double total_parameter_sizes = mt.parameter_sizes[0][mt.parameter_sizes[0].size() - 1];
  cout << "total_parameter_sizes:" << total_parameter_sizes << endl;
  auto n_wids = get<2>(d.bitnext(empty, num_machines)[0]);
  for (auto s : n_wids) cout << s << " ";
  double total_communication_time = DataParallel(d, n_wids, total_parameter_sizes);
  cout << "total_dp_communication_time:" << total_communication_time << endl;
  cout << "Current DP Time: " << total_compute_time + total_communication_time << endl;
  cout << "DP Theoretical Speedup: "
       << DPSpeedup(this->d, total_compute_time, total_communication_time) << endl;
}

TA Conductor::compute_spa(int spa_size, Devices sd) {
  TA   A;
  auto compute_times           = this->m.Meta.compute_times;
  auto activation_sizes        = this->m.Meta.activation_sizes;
  auto parameter_sizes         = this->m.Meta.parameter_sizes;
  auto output_activation_sizes = this->m.Meta.output_activation_sizes;
  auto all_predecessor_ids     = this->m.Meta.all_predecessor_ids;
  auto d                       = sd;
  auto num_machines            = spa_size;

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

  int rp = 1;  // as if we only have one pipeline
  // DP Initialization
  frs(j, 0, compute_times[0].size()) {
    double cur_compute_time    = compute_times[0][j];
    double cur_activation_size = activation_sizes[0][j];
    double cur_parameter_size  = parameter_sizes[0][j];
    int    max_m               = num_machines;  // TODO: check straight_pipeline
    fri(m, 0, max_m) {
      // auto bs_after_m = std::get<1>(d.bitnext(empty, m + 1)[0]);
      auto [n_switch, n_bitset, n_wids] = d.bitnext(empty, m + 1)[0];
#ifdef DEBUG
      cout << "n_wids = [";
      for (auto i : n_wids) cout << i << " ";
      cout << "]" << endl;
#endif
      if (cur_compute_time < -0.5) {  // normally -1
        A[rp][j][m][ph]       = make_tuple(-1, make_pair(-1, -1), -1, ph, set<int>{});
        A[rp][j][m][n_bitset] = make_tuple(-1, make_pair(-1, -1), -1, ph, n_wids);
      } else {
        double cur_dp_time = std::max((cur_compute_time) / (m + 1), 0.0) +
                             DataParallel(d, n_wids, cur_parameter_size);
#ifdef DEBUG
        cout << "InitA j = " << j << " m = " << m << " : " << cur_dp_time << endl;
#endif
        A[rp][j][m][ph] = make_tuple(cur_dp_time, make_pair(-1, -1), m + 1, empty, set<int>{});
        A[rp][j][m][n_bitset] = make_tuple(cur_dp_time, make_pair(-1, -1), m + 1, empty, n_wids);

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
      // double         min_pipeline_time;
      // pair<int, int> optimal_split;
      // int            optimal_num_machines;
      // ll             optimal_bitset;
      // set<int>       last_machines;
      if (A[rp][j][m].find(ph) == A[rp][j][m].end()) {
        continue;
      }

      auto [min_pipeline_time, optimal_split, optimal_num_machines, last_from, last_machines] =
          A[rp][j][m][ph];

      for (auto k : all_predecessor_ids[j]) {  // NOTE: iterate over j's predecessors
        int max_mp = m + 1;                    // TODO: straight pipeline check
        for (auto mp = 1; mp < max_mp; mp++) {
          for (auto& bs : A[rp][k][m - mp]) {
            if (bs.first.size() > num_machines) continue;  // skip ph

#ifdef DEBUGBS
            // for the current bs, print the bitset
            cout << "For BS: ";
            for (auto b : bs.first) cout << b;
            cout << endl;
#endif

            auto prev_bs     = bs.first;
            auto next_bs_all = d.bitnext(prev_bs, mp);
#ifdef DEBUGBS
            // print all the next available bs machines array
            for (auto s : next_bs_all) {
              cout << get<0>(s) << ": ";
              for (auto n : get<2>(s)) cout << n << " ";
            }
            cout << endl;
#endif

            for (auto nbs : next_bs_all) {
              auto [n_switch, n_bitset, n_wids] = nbs;  // NOTE: [FF, SF, AF], v<b>, v<i>

#ifdef DEBUG
              cout << "FROM [";
              for (auto b : get<4>(bs.second)) cout << b << " ";
              cout << "]" << endl << "TO [";
              for (auto b : n_wids) cout << b << " ";
              cout << "]" << endl;
#endif

              set<int> from = get<4>(bs.second), to = n_wids;

              // double input_transfer_time = (2.0 * output_activation_sizes[k]) /
              //                              (B_ETHERNET * mp);  // TODO: hardcoded for debugging
              double input_transfer_time =
                  SplitConcat(d, from, to, 2.0 * output_activation_sizes[k]);
              double output_transfer_time = -1;
              // if (j < output_activation_sizes.size() - 1) {
              //   output_transfer_time = (2.0 * output_activation_sizes[j]) /
              //                          (B_ETHERNET * mp);  // TODO: hardcoded for debugging
              // }

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
              if (output_transfer_time > -0.5)  // NOTE: nevern triggered due to comments above
                pipeline_time = max(pipeline_time, output_transfer_time);

              if (min_pipeline_time < -0.5 || min_pipeline_time > pipeline_time) {
                optimal_split        = make_pair(k, m - mp);
                optimal_num_machines = mp;
                min_pipeline_time    = pipeline_time;
                last_from            = prev_bs;
                last_machines        = n_wids;
#ifdef DEBUG
                cout << "new min_pipeline_time: " << min_pipeline_time << endl;
#endif
              }

              if (A[rp][j][m].find(n_bitset) == A[rp][j][m].end() ||
                  pipeline_time <
                      get<0>(A[rp][j][m][n_bitset])) {  // abusing the short-circuit here
                A[rp][j][m][n_bitset] =
                    make_tuple(pipeline_time, make_pair(k, m - mp), mp, prev_bs, n_wids);
              }
              // A[rp][j][m][n_bitset] =
              //     make_tuple(min_pipeline_time, optimal_split, optimal_num_machines,
              //     last_machines);
            }
          }
        }
      }
      A[rp][j][m][ph] =
          make_tuple(min_pipeline_time, optimal_split, optimal_num_machines, last_from, set<int>{});
    }
  }

  return A;
}

TA Conductor::compute_partitioning() { return A; }

TA Conductor::compute_partitioning(d2d compute_times, d2d activation_sizes, d2d parameter_sizes,
                                   vector<double> output_activation_sizes,
                                   i2d            all_predecessor_ids) {
  TA  A;
  int num_machines = this->d.G;

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
  Devices d  = this->d;
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
#ifdef DEBUG
      cout << "n_wids = [";
      for (auto i : n_wids) cout << i << " ";
      cout << "]" << endl;
#endif
      if (cur_compute_time < -0.5) {  // normally -1
        A[rp][j][m][ph]       = make_tuple(-1, make_pair(-1, -1), -1, ph, set<int>{});
        A[rp][j][m][n_bitset] = make_tuple(-1, make_pair(-1, -1), -1, ph, n_wids);
      } else {
#ifdef DEBUG
        cout << "InitA j = " << j << " m = " << m << " : "
             << std::max((cur_compute_time) / (m + 1), 0.0) +
                    DataParallel(d, n_wids, cur_parameter_size)
             << endl;
#endif
        A[rp][j][m][ph] = make_tuple(
            std::max((cur_compute_time) / (m + 1), 0.0 /*cur_parameter_size * 1 / B_ETHERNET*/) +
                DataParallel(d, n_wids, cur_parameter_size),
            make_pair(-1, -1), m + 1, empty, set<int>{});
        A[rp][j][m][n_bitset] = make_tuple(std::max((cur_compute_time) / (m + 1), 0.0) +
                                               DataParallel(d, n_wids, cur_parameter_size),
                                           make_pair(-1, -1), m + 1, empty, n_wids);

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
      // double         min_pipeline_time;
      // pair<int, int> optimal_split;
      // int            optimal_num_machines;
      // ll             optimal_bitset;
      // set<int>       last_machines;
      if (A[rp][j][m].find(ph) == A[rp][j][m].end()) {
        continue;
      }

      auto [min_pipeline_time, optimal_split, optimal_num_machines, last_from, last_machines] =
          A[rp][j][m][ph];

      for (auto k : all_predecessor_ids[j]) {  // NOTE: iterate over j's predecessors
        int max_mp = m + 1;                    // TODO: straight pipeline check
        for (auto mp = 1; mp < max_mp; mp++) {
          for (auto& bs : A[rp][k][m - mp]) {
            if (bs.first.size() > num_machines) continue;  // skip ph

#ifdef DEBUGBS
            // for the current bs, print the bitset
            cout << "For BS: ";
            for (auto b : bs.first) cout << b;
            cout << endl;
#endif

            auto prev_bs     = bs.first;
            auto next_bs_all = d.bitnext(prev_bs, mp);
#ifdef DEBUGBS
            // print all the next available bs machines array
            for (auto s : next_bs_all) {
              cout << get<0>(s) << ": ";
              for (auto n : get<2>(s)) cout << n << " ";
            }
            cout << endl;
#endif

            for (auto nbs : next_bs_all) {
              auto [n_switch, n_bitset, n_wids] = nbs;  // NOTE: [FF, SF, AF], v<b>, v<i>

#ifdef DEBUG
              cout << "FROM [";
              for (auto b : get<4>(bs.second)) cout << b << " ";
              cout << "]" << endl << "TO [";
              for (auto b : n_wids) cout << b << " ";
              cout << "]" << endl;
#endif

              set<int> from = get<4>(bs.second), to = n_wids;

              // double input_transfer_time = (2.0 * output_activation_sizes[k]) /
              //                              (B_ETHERNET * mp);  // TODO: hardcoded for debugging
              double input_transfer_time =
                  SplitConcat(d, from, to, 2.0 * output_activation_sizes[k]);
              double output_transfer_time = -1;
              // if (j < output_activation_sizes.size() - 1) {
              //   output_transfer_time = (2.0 * output_activation_sizes[j]) /
              //                          (B_ETHERNET * mp);  // TODO: hardcoded for debugging
              // }

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
              if (output_transfer_time > -0.5)  // NOTE: nevern triggered due to comments above
                pipeline_time = max(pipeline_time, output_transfer_time);

              if (min_pipeline_time < -0.5 || min_pipeline_time > pipeline_time) {
                optimal_split        = make_pair(k, m - mp);
                optimal_num_machines = mp;
                min_pipeline_time    = pipeline_time;
                last_from            = prev_bs;
                last_machines        = n_wids;
#ifdef DEBUG
                cout << "new min_pipeline_time: " << min_pipeline_time << endl;
#endif
              }

              if (A[rp][j][m].find(n_bitset) == A[rp][j][m].end() ||
                  pipeline_time <
                      get<0>(A[rp][j][m][n_bitset])) {  // abusing the short-circuit here
                A[rp][j][m][n_bitset] =
                    make_tuple(pipeline_time, make_pair(k, m - mp), mp, prev_bs, n_wids);
              }
              // A[rp][j][m][n_bitset] =
              //     make_tuple(min_pipeline_time, optimal_split, optimal_num_machines,
              //     last_machines);
            }
          }
        }
      }
      A[rp][j][m][ph] =
          make_tuple(min_pipeline_time, optimal_split, optimal_num_machines, last_from, set<int>{});
    }
  }
  return A;
}

std::vector<std::tuple<int, int, int, set<int>>> Conductor::analyse_partitioning(TA A, int end, int num_machines) {
  std::vector<std::tuple<int, int, int, set<int>>> res;
  int                                              rp           = 1;  // NOTE: hardcoded
  ll                                               ph;
  for (int i = 0; i < num_machines; i++) ph.push_back(true);

  std::cout << "\033[33mEnter HPGO Analyse Partitioning\033[0m" << std::endl;
  auto             metadata                = A[rp][end - 1][num_machines - 1][ph];
  auto             next_split              = std::get<1>(metadata);
  auto             last_machines           = std::get<4>(metadata);
  auto             last_from               = std::get<3>(metadata);
  int              remaining_machines_left = num_machines;
  std::vector<int> splits;
  std::vector<int> replication_factors;
  int              prev_split = end - 1;

  std::cout << "\033[33m" << next_split.first << ", " << next_split.second << "\033[0m"
            << std::endl;

  while (next_split.first != -1) {  // -1 means None
    int num_machines_used = std::get<2>(metadata);

    res.push_back(make_tuple(prev_split, next_split.first + 1, num_machines_used, last_machines));
    prev_split = get<1>(res[res.size() - 1]);

    metadata      = A[rp][next_split.first][next_split.second][last_from];
    next_split    = std::get<1>(metadata);
    last_from     = std::get<3>(metadata);
    last_machines = std::get<4>(metadata);
    replication_factors.push_back(num_machines_used);
    remaining_machines_left -= num_machines_used;
    std::cout << "\033[33m" << next_split.first << ", " << next_split.second << "\033[0m"
              << std::endl;
  }

  int num_machines_used = std::get<2>(metadata);
  remaining_machines_left -= num_machines_used;

  res.push_back(make_tuple(0, prev_split, num_machines_used, last_machines));
  //  std::cout << "\033[33m-------------------------------------\033[0m" << std::endl;
  //  std::cout << "\033[33mnum_machines_used: " << num_machines_used << "\033[0m" << std::endl;

  std::reverse(res.begin(), res.end());
  // prev_split = 0;
  // std::reverse(splits.begin(), splits.end());
  // splits.push_back(end);
  // replication_factors.push_back(num_machines_used);
  // std::reverse(replication_factors.begin(), replication_factors.end());
  return res;
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
          auto [min_pipeline_time, optimal_split, optimal_num_machines, last_from, wids] =
              bs.second;
          cout << "[" << min_pipeline_time << ", \t(" << optimal_split.first << ", "
               << optimal_split.second << "), \t" << optimal_num_machines << ", \t(";
          for (auto b : last_from) cout << b;
          cout << ")\t(";
          for (auto s : wids) {
            cout << s << " ";
          }
          cout << ")]" << endl;
        }
      }
    }
  }
}

void Conductor::printA() { printA(this->A); }

TA Conductor::getA() { return this->A; }

void Conductor::setProfileFilename(std::string fn) {
  this->profile_filename = fn;
  Graph g                = Graph(fn);
  this->setGraph(g);
  Model m = Model(1024, 32, g);  // FIXME: hardcoded gbs and pbs
  // m.Normalize();
  // m.SetLayerStats(g.compute_times, g.activation_sizes, g.parameter_sizes,
  //                 g.output_activation_sizes);
  this->setModel(m);  // TODO: interface for global batch size
}
void Conductor::setModel(Model m) { this->m = m; }
void Conductor::setGraph(Graph g) { this->g = g; }
void Conductor::setDevices(Devices d) { this->d = d; }
