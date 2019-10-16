#include <cmath>
#include <iostream>
#include <string>
#include "../model.h"

#include "catch.hpp"
#include "test_config.h"
using namespace std;

TEST_CASE("Model Re-construction with Python States", "[PY]") {
  Graph g = Graph("../profiler/image_classification/profiles/vgg16/graph.txt");
  Model m = Model(512, g.getStates());
  for (auto l : m.States) {
    cout << "State #" << l.ID << ", " << l.Name << ", " << l.Desc << ", C=" << l.CompTime
         << ", A=" << l.ActivationSize << ", OA=" << l.OutputActivationSize << ", P=" << l.ParamSize
         << endl;
  }
}