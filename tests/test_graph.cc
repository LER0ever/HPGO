#include <cmath>
#include <iostream>
#include <string>
#include "../input/graph.h"

#include "catch.hpp"
#include "test_config.h"
using namespace std;

TEST_CASE("Graph interop with Python", "[PY]") {
  SECTION("Construct a Py Graph object") {
    Graph g = Graph("../profiler/image_classification/profiles/vgg16/graph.txt");
  }
}