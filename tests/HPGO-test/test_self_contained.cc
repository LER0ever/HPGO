#include <HPGO/environment/device.h>
#include <HPGO/model/model.h>
#include <HPGO/orchestration/orchestration.h>
#include <HPGO/utils/helper.h>
#include <bits/stdc++.h>
#include <gmock/gmock.h>

#include "test_config.h"

using namespace std;
using ll     = vector<bool>;
using triple = tuple<double, pair<int, int>, int>;
using TypeA  = vector<vector<vector<triple>>>;
using d2d    = vector<vector<double>>;
using i2d    = vector<vector<int>>;

class AlgoSelfContainedTest : public testing::Test {
 public:
};

TEST_F(AlgoSelfContainedTest, ASCMain) {
  // Hardcode VGG data
  cout << "Reading Profiling Graph TXT..." << endl;
  Graph g = Graph("./profiler/image_classification/profiles/vgg19/graph.txt");
  Model m = Model(1024, 32, g);
  auto mt = m.Meta;
  // m.SetLayerStats(g.compute_times, g.activation_sizes, g.parameter_sizes,
  //                 g.output_activation_sizes);
  for (auto l : m.Layers) {
    cout << "Layer #" << l.ID << ", " << l.Name << ", C=" << l.CompTime
         << ", A=" << l.ActivationSize << ", OA=" << l.OutputActivationSize << ", P=" << l.ParamSize
         << endl;
  }

  double total_compute_times    = mt.compute_times[0][mt.compute_times[0].size() - 1];
  double total_parameter_sizes  = mt.parameter_sizes[0][mt.parameter_sizes[0].size() - 1];
  double total_activation_sizes = mt.activation_sizes[0][mt.activation_sizes[0].size() - 1];
  cout << "Total Computational Time: " << total_compute_times << endl;
  cout << "Total Parameter Size: " << total_parameter_sizes << endl;
  cout << "Total Activation Size: " << total_activation_sizes << endl;

  frs(i, 0, mt.all_predecessor_ids.size()) {
    cout << i << ": ";
    for (auto id2 : mt.all_predecessor_ids[i]) {
      cout << id2 << " ";
    }
    cout << endl;
  }

  Devices d = Devices(3, std::vector<int>{2, 3});

  Conductor C;
  C.setProfileFilename("./profiler/image_classification/profiles/vgg19/graph.txt");
  C.setDevices(d);
  C.orchestrate();
  // C.orchestrate(16, vector<int>{16});

  auto A = C.compute_partitioning(mt.compute_times, mt.activation_sizes, mt.parameter_sizes, mt.output_activation_sizes, mt.all_predecessor_ids);
  C.printA(A);

  // auto A = C.compute_spa(3, sd);
  // C.printA(A);
  auto res = C.analyse_partitioning(A[1], g.compute_times[0].size(), 3,1);
  for (int i=0; i<res.size(); i++){
    cout << "(" << get<0>(res[i]) << " ~ " << get<1>(res[i]) << ") x " << get<2>(res[i]) << " @ [";
    auto wids = get<3>(res[i]);
    for (auto s : wids) cout << " " << s;
    cout << " ]" << endl;
  }
}

TEST_F(AlgoSelfContainedTest, ASCMain_MultiLevel) {
  // Hardcode VGG data
  cout << "Reading Profiling Graph TXT..." << endl;
  Graph g = Graph("./profiler/translation/profiles/gnmt/graph.txt");
  Model m = Model(1024, 32, g);
  auto mt = m.Meta;
  // m.SetLayerStats(g.compute_times, g.activation_sizes, g.parameter_sizes,
  //                 g.output_activation_sizes);
  for (auto l : m.Layers) {
    cout << "Layer #" << l.ID << ", " << l.Name << ", C=" << l.CompTime
         << ", A=" << l.ActivationSize << ", OA=" << l.OutputActivationSize << ", P=" << l.ParamSize
         << endl;
  }

  double total_compute_times    = mt.compute_times[0][mt.compute_times[0].size() - 1];
  double total_parameter_sizes  = mt.parameter_sizes[0][mt.parameter_sizes[0].size() - 1];
  double total_activation_sizes = mt.activation_sizes[0][mt.activation_sizes[0].size() - 1];
  cout << "Total Computational Time: " << total_compute_times << endl;
  cout << "Total Parameter Size: " << total_parameter_sizes << endl;
  cout << "Total Activation Size: " << total_activation_sizes << endl;

  frs(i, 0, mt.all_predecessor_ids.size()) {
    cout << i << ": ";
    for (auto id2 : mt.all_predecessor_ids[i]) {
      cout << id2 << " ";
    }
    cout << endl;
  }

  Devices d = Devices(16, std::vector<int>{8, 16});

  Conductor C;
  C.setProfileFilename("./profiler/translation/profiles/gnmt/graph.txt");
  C.setDevices(d);
  C.orchestrate();
  // C.orchestrate(16, vector<int>{16});

  auto A = C.compute_partitioning(2, 8);
  C.printSA(A);

  // auto A = C.compute_spa(3, sd);
  // C.printA(A);

  auto res = C.analyse_partitioning(A, g.compute_times[0].size(), 2, 8);
  for (int i=0; i<res.size(); i++){
    cout << "(" << get<0>(res[i]) << " ~ " << get<1>(res[i]) << ") x " << get<2>(res[i]) << " @ [";
    auto wids = get<3>(res[i]);
    for (auto s : wids) cout << " " << s;
    cout << " ]" << endl;
  }
}
