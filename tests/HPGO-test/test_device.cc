#include <HPGO/environment/device.h>
#include <gmock/gmock.h>
#include <cmath>
#include <iostream>
#include <string>

#include "test_config.h"
using namespace std;

class DeviceTest : public testing::Test {
 public:
};

TEST_F(DeviceTest, DeviceInfoConstruction) {
  // fiblib::Fibonacci fib;
  Device d = Device(vector<int>{2, 8}, vector<double>{3.0 * 1000000000, 25.0 * 1000000000});
  EXPECT_EQ(d.SubDevices.size(), 2);
  EXPECT_EQ(d.SubDevices[0].SubDevices.size(), 8);
}

TEST_F(DeviceTest, DeviceBitsetOperations_7_7) {
  Devices d = Devices(16, vector<int>{8, 16});
  cout << "Request 7 GPUs" << endl;
  auto ret = d.bitnext(d.dbs, 7);
  for (auto s : ret) {
    cout << get<0>(s) << ": ";
    for (auto n : get<2>(s)) cout << n << " ";
  }
  cout << endl;
  cout << "Request 7 GPUs" << endl;
  auto new_bs = get<1>(ret[0]);
  ret         = d.bitnext(new_bs, 7);
  for (auto s : ret) {
    cout << get<0>(s) << ": ";
    for (auto n : get<2>(s)) cout << n << " ";
  }
  cout << endl;
}

TEST_F(DeviceTest, DeviceBitsetOperations_15_1) {
  Devices d = Devices(16, vector<int>{8, 16});
  cout << "Request 15 GPUs" << endl;
  auto ret = d.bitnext(d.dbs, 15);
  for (auto s : ret) {
    cout << get<0>(s) << ": ";
    for (auto n : get<2>(s)) cout << n << " ";
  }
  cout << endl;
  cout << "Request 1 GPUs" << endl;
  auto new_bs = get<1>(ret[0]);
  ret         = d.bitnext(new_bs, 1);
  for (auto s : ret) {
    cout << get<0>(s) << ": ";
    for (auto n : get<2>(s)) cout << n << " ";
  }
  cout << endl;
}

TEST_F(DeviceTest, IsCrossMachineTest) {
  Devices d = Devices(16, vector<int>{8, 16});
  EXPECT_EQ(d.is_cross_machine(set<int>{1,2,3,4}, set<int>{9,10,11,12}), true);
  EXPECT_EQ(d.is_cross_machine(set<int>{1,2,3,4}, set<int>{5,6,7,8}), false);
  EXPECT_EQ(d.is_cross_machine(set<int>{1,2,3,4}, set<int>{5,6,7,8,9}), true);
}
