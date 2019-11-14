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

void print2DBitNext(vector<vector<BNRet>> ret)
{
  for (auto sbn: ret) {
    for (auto bn : sbn) {
      cout << "[";
      for (auto wid : get<2>(bn)) cout << wid << " ";
      cout << "] ";
    }
    cout << endl;
  }
}

void print1DBitNext(vector<BNRet> ret)
{
  for (auto s : ret) {
    cout << get<0>(s) << ": (";
    for (auto b : get<1>(s)) cout << b;
    cout << ") - [";
    for (auto n : get<2>(s)) cout << n << " ";
    cout << "]" << endl;
  }
  cout << endl;
}

TEST_F(DeviceTest, DeviceBitsetOperations_DP) {
  Devices d = Devices(16, vector<int>{8, 16});
  cout << "\nRequest 1 * 3 GPUs" << endl;
  auto ret = d.bitnext(d.dbs, 1, 3);
  print2DBitNext(ret);
  cout << "\nRequest 2 * 3 GPUs" << endl;
  ret = d.bitnext(d.dbs, 2, 3);
  print2DBitNext(ret);
  cout << "\nRequest 4 * 4 GPUs" << endl;
  ret = d.bitnext(d.dbs, 4, 4);
  print2DBitNext(ret);
  cout << "\nRequest 4 * 4 GPUs and merge them" << endl;
  auto mergedRet = d.bnmerge(ret);
  print1DBitNext(mergedRet);
  cout << "\nRequest 3 * 5 GPUs" << endl;
  ret = d.bitnext(d.dbs, 3, 5);
  print2DBitNext(ret);
  cout << "\nRequest 2 * 5 GPUs, then 1 * 5 on the first result" << endl;
  cout << "========" << endl;
  ret = d.bitnext(d.dbs, 2, 5);
  print2DBitNext(ret);
  mergedRet = d.bnmerge(ret);
  print1DBitNext(mergedRet);
  cout << "--------" << endl;
  auto firstbs = get<1>(ret[0][4]);
  ret = d.bitnext(firstbs, 1, 5);
  print2DBitNext(ret);
  cout << "========" << endl;

  // TODO: test bnmerge(bitnext(x, x, 1)) -- bitnext(x, x)
}

TEST_F(DeviceTest, IsCrossMachineTest) {
  Devices d = Devices(16, vector<int>{8, 16});
  EXPECT_EQ(d.is_cross_machine(set<int>{1,2,3,4}, set<int>{9,10,11,12}), true);
  EXPECT_EQ(d.is_cross_machine(set<int>{0,1,2,3}, set<int>{4,5,6,7}), false);
  EXPECT_EQ(d.is_cross_machine(set<int>{0,1,2,3}, set<int>{4,5,6,7,8}), true);
  EXPECT_EQ(d.is_cross_machine(set<int>{1,2,3,4}, set<int>{5,6,7,8,9}), true);
  d = Devices(3, vector<int>{2,3});
  EXPECT_EQ(d.is_cross_machine(set<int>{0,1,2}), true);
}
