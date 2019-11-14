#include <HPGO/environment/device.h>
#include <HPGO/model/model.h>
#include <HPGO/orchestration/orchestration.h>
#include <HPGO/parallelism/data-parallel/data-parallel.h>
#include <HPGO/utils/helper.h>
#include <bits/stdc++.h>
#include <gmock/gmock.h>

#include "test_config.h"

using namespace std;

class DataParallelTest : public testing::Test {
 public:
};

TEST_F(DataParallelTest, TransferTimeCalculation) {
  Devices d = Devices(3, vector<int>{2, 3});

  double dp_time = DataParallel(d, set<int>{0, 1, 2}, 1000000000);
  EXPECT_LE(abs(dp_time - 4.0/9), EPSILON);
}

TEST_F(DataParallelTest, SpeedupCalculation) {
  Devices d = Devices(16, vector<int>{8, 16});
  double comp_time = 4.61472;
  double comm_time = 0.342529;
  double speedup = DPSpeedup(d, comp_time, comm_time);
  EXPECT_LE(abs(speedup - 7.31394), EPSILON);
}
