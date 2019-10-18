#include <cmath>
#include <iostream>
#include <string>
#include "../model/model.h"

#include "catch.hpp"
#include "test_config.h"
using namespace std;

TEST_CASE("Model Re-construction with Python States", "[PY]") {
  Graph g = Graph("../profiler/image_classification/profiles/vgg16/graph.txt");
  Model m = Model(512, g);
  for (auto l : m.States) {
    cout << "State #" << l.ID << ", " << l.Name << ", C=" << l.CompTime
         << ", A=" << l.ActivationSize << ", OA=" << l.OutputActivationSize << ", P=" << l.ParamSize
         << endl;
  }
  m.SetLayerStats(g.compute_times, g.activation_sizes, g.parameter_sizes,
                  g.output_activation_sizes);
  for (auto l : m.Layers) {
    cout << "Layer #" << l.ID << ", " << l.Name << ", C=" << l.CompTime
         << ", A=" << l.ActivationSize << ", OA=" << l.OutputActivationSize << ", P=" << l.ParamSize
         << endl;
  }
}