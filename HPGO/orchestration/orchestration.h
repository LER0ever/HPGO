#ifndef ORCHESTRATION_H
#define ORCHESTRATION_H

#include <HPGO/HPGO_api.h>
#include <HPGO/environment/device.h>
#include <HPGO/input/graph.h>
#include <HPGO/model/model.h>
#include <HPGO/parallelism/pipeline/duration.h>
#include <HPGO/parallelism/pipeline/syncpipeline.h>
#include <set>
#include <tuple>
#include <unordered_map>
#include <vector>

using d2d = std::vector<std::vector<double>>;
using i2d = std::vector<std::vector<int>>;
using ll  = std::vector<bool>;
using EA  = std::tuple<double, std::pair<int, int>, int, ll, std::set<int>>;
using TA  = std::vector<std::vector<std::vector<std::unordered_map<ll, EA>>>>;
using SA = std::vector<std::vector<std::unordered_map<ll, EA>>>;
using MA = std::unordered_map<int, SA>;

class HPGO_API Conductor {
 public:
  void setModel(Model);
  void setGraph(Graph);
  void setDevices(Devices);
  void setProfileFilename(std::string, int, int, int);
  void orchestrate();

  SA compute_partitioning(int spa_size, int rp);
  std::vector<std::tuple<int, int, int, std::set<int>>> analyse_partitioning(SA A, int end, int num_machines, int rp);

  // Debug Helper
  void printA(TA& A);
  void printSA(SA& A);
  void printA();
  TA getA();
  Graph getGraph();

 private:
  Model       m;
  TA          A;
  Graph       g;
  Devices     d;
  std::string profile_filename;
};

// Hierarchical Based Orchestration
using triple = std::tuple<double, std::pair<int, int>, int>;
using TypeA  = std::vector<std::vector<std::vector<triple>>>;

class HPGO_API HierarchicalConductor {
 public:
  void setModel(Model);
  void setGraph(Graph);
  void setProfileFilename(std::string);
  void orchestrate(std::vector<int> all_num_machines, std::vector<double> all_bandwidths);
  void orchestrate_single(std::vector<int> all_num_machines, std::vector<double> all_bandwidths,
                          int replica, double dp_bandwidth);

  TypeA compute_partitioning(d2d compute_times, d2d activation_sizes, d2d parameter_sizes,
                             std::vector<double> output_activation_sizes, i2d all_predecessor_ids,
                             int num_machines, int num_machines_within_machine, double bandwidth,
                             bool final_level);
  std::vector<int> analyse_partititioning(TypeA A, int start, int end, double network_bandwidth,
                                          int num_machines);

  // Debug Helper
  void printA(TypeA& A);

 private:
  Model       m;
  TypeA       A;
  Graph       g;
  std::string profile_filename;
};

#endif  // ORCHESTRATION_H
