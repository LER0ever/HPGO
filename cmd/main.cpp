
#include <HPGO/environment/device.h>
#include <HPGO/model/model.h>
#include <HPGO/orchestration/orchestration.h>
#include <HPGO/utils/helper.h>
#include <bits/stdc++.h>
#include <cassert>

using namespace std;

int main(int argc, char* argv[])
{
  if (argc <= 5) {
    cout <<  "HPGO-CMD profile_path global_batch_size profile_batch_size min_micro_batch_size wids[0] wids[1]\ne.g. HPGO-CMD ./profile/xxx/vgg19.txt 4096 32 32 8 16" << endl;
    assert(0 && "argc check failed");
  }
  string profile_path(argv[1]);
  int gbs = atoi(argv[2]);
  int pbs = atoi(argv[3]);
  int mbs = atoi(argv[4]);
  vector<int> wids;
  for (int i=5; i<argc; i++) {
    wids.push_back(atoi(argv[i]));
  }
  // Hardcode VGG data
  cout << "Reading Profiling Graph TXT..." << endl;
//  Graph g  = Graph("./profiler/image_classification/profiles/vgg19/graph.txt");
  Graph g  = Graph(profile_path);
  Model m  = Model(gbs, pbs, mbs, g);
  auto  mt = m.Meta;
  // m.SetLayerStats(g.compute_times, g.activation_sizes, g.parameter_sizes,
  //                 g.output_activation_sizes);
  for (auto l : m.Layers) {
    cout << "Layer #" << l.ID << ", " << l.Desc << ", C=" << l.CompTime
         << ", A=" << l.ActivationSize << ", OA=" << l.OutputActivationSize << ", P=" << l.ParamSize
         << endl;
  }

  double total_compute_times    = mt.compute_times[0][mt.compute_times[0].size() - 1];
  double total_parameter_sizes  = mt.parameter_sizes[0][mt.parameter_sizes[0].size() - 1];
  double total_activation_sizes = mt.activation_sizes[0][mt.activation_sizes[0].size() - 1];
  cout << "Total Computational Time: " << total_compute_times << " s."<< endl;
  cout << "Total Parameter Size: " << total_parameter_sizes << " Bytes." << endl;
  cout << "Total Activation Size: " << total_activation_sizes << " Bytes" << endl;

  frs(i, 0, mt.all_predecessor_ids.size()) {
    cout << i << ": ";
    for (auto id2 : mt.all_predecessor_ids[i]) {
      cout << id2 << " ";
    }
    cout << endl;
  }

  Devices d = Devices(wids[wids.size()-1], wids);

  Conductor C;
  C.setProfileFilename(profile_path, gbs, pbs, mbs);
  C.setDevices(d);
  C.orchestrate();
}
