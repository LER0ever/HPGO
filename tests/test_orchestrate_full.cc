#include <cmath>
#include <iostream>
#include <string>
#include "../graph.h"
#include "../orchestration.h"

#include "catch.hpp"
#include "test_config.h"
using namespace std;

TEST_CASE("|| Full VGG16 2x8 Test ||", "[GO]") {
  Conductor C;
  C.orchestrate(vector<int>{8, 2}, vector<double>{25.0 * 1000000000, 3.0 * 1000000000},
                "../profiler/image_classification/profiles/vgg16/graph.txt");
}