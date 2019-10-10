#include <cmath>
#include <iostream>
#include "../orchestration.h"
#include "../syncpipeline.h"

#include "catch.hpp"
#include "test_config.h"
using namespace std;

TEST_CASE("Global Orchestration Tests", "[GO]") {
  vector<double> F(4, 1);
  vector<double> B(4, 2);
  vector<double> OF(4, 33.333333);
  vector<double> OB(4, 33.333333);
  auto           sp = SyncPipeline(7, F, OF, B, OB);
  sp.getSingleLength();
  cout << "getSingleLength() done" << endl;
  vector<vector<vector<Block> > > blk = sp.getBlock();
  for (int i = 0; i < blk[0].size(); i++)
    for (int j = 0; j < blk[0][i].size(); j++)
      cout << "Block[0][" << i << "][" << j
           << "].end() = " << blk[0][i][j].getDuration().getEnd() << endl;

  auto sl = sp.getSingleLengthAnalytical();

  cout << "sp.getSingleLengthAnalytical() = " << sl << endl;
  REQUIRE(abs(sl - 32.0) < 0.01);
}
