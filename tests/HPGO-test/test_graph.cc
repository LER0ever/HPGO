#include <HPGO/input/graph.h>
#include <gmock/gmock.h>
#include <cmath>
#include <iostream>
#include <string>

#include "test_config.h"
using namespace std;

class GraphInputTest : public testing::Test {};

TEST_F(GraphInputTest, GraphFromPDInput)
{
  Graph g = Graph("./profiler/image_classification/profiles/vgg16/graph.txt");
}
