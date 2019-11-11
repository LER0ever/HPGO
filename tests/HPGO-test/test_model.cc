#include <HPGO/model/model.h>
#include <gmock/gmock.h>
#include <cmath>
#include <iostream>
#include <string>

#include "test_config.h"
using namespace std;

class ModelTest : public testing::Test {};

TEST_F(ModelTest, TestModelConstruction) {
  Graph g = Graph("./profiler/image_classification/profiles/vgg16/graph.txt");
  Model m = Model(512, 128, g);
  for (auto l : m.States) {
    cout << "State #" << l.ID << ", " << l.Name << ", C=" << l.CompTime
         << ", A=" << l.ActivationSize << ", OA=" << l.OutputActivationSize << ", P=" << l.ParamSize
         << endl;
  }
  // m.SetLayerStats(g.compute_times, g.activation_sizes, g.parameter_sizes,
                  // g.output_activation_sizes);
  for (auto l : m.Layers) {
    cout << "Layer #" << l.ID << ", " << l.Name << ", C=" << l.CompTime
         << ", A=" << l.ActivationSize << ", OA=" << l.OutputActivationSize << ", P=" << l.ParamSize
         << endl;
  }
}
