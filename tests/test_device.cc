#include <cmath>
#include <iostream>
#include <string>
#include "../environment/device.h"

#include "catch.hpp"
#include "test_config.h"
using namespace std;

TEST_CASE("Device information construction", "[ENV]") {
  Device d = Device(vector<int>{2, 8}, vector<double>{3.0 * 1000000000, 25.0 * 1000000000});
  REQUIRE(d.SubDevices.size() == 2);
  REQUIRE(d.SubDevices[0].SubDevices.size() == 8);
}