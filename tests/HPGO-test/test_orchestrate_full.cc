#include <HPGO/input/graph.h>
#include <HPGO/orchestration/orchestration.h>
#include <gmock/gmock.h>
#include <cmath>
#include <iostream>
#include <string>

#include "test_config.h"
using namespace std;

class OrchestrationTest : public testing::Test {};

TEST_F(OrchestrationTest, TestFullOrchestration) {
  HierarchicalConductor C;
  C.setProfileFilename("./profiler/image_classification/profiles/vgg16/graph.txt");
  C.orchestrate(vector<int>{8, 2}, vector<double>{25.0 * 1000000000, 0.8 * 1000000000});
}
