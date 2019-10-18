#include <cmath>
#include <iostream>
#include "../orchestration/orchestration.h"
#include "../parallelism/pipeline/syncpipeline.h"

#include "catch.hpp"
#include "test_config.h"
using namespace std;

TEST_CASE("Pipeline Efficiency Calculation Test", "[PEC]") {
  SECTION("4 stage pipeline with F:B=1:2, mBatch = 7") {
    vector<double> F(4, 1);
    vector<double> B(4, 2);
    vector<double> OF(4, 33.333333);
    vector<double> OB(4, 33.333333);
    auto           sp = SyncPipeline(7, F, OF, B, OB);
    sp.getSingleLength();
    everMsg("getSingleLength() done");
    vector<vector<vector<Block> > > blk = sp.getBlock();
    for (int i = 0; i < blk[0].size(); i++)
      for (int j = 0; j < blk[0][i].size(); j++)
        dout << "Block[0][" << i << "][" << j << "].end() = " << blk[0][i][j].getDuration().getEnd()
             << endl;

    auto sl = sp.getSingleLengthAnalytical();

    dout << "sp.getSingleLengthAnalytical() = " << sl << endl;
    REQUIRE(abs(sl - 32.0) < EPSILON);
  }

  SECTION("2 stage pipeline with F:B=1:2, mBatch = 3") {
    vector<double> F(2, 1);
    vector<double> B(2, 2);
    vector<double> OF(2, 33.333333);
    vector<double> OB(2, 33.333333);
    auto           sp = SyncPipeline(3, F, OF, B, OB);
    sp.getSingleLength();
    dout << "getSingleLength() done" << endl;
    vector<vector<vector<Block> > > blk = sp.getBlock();
    for (int i = 0; i < blk[0].size(); i++)
      for (int j = 0; j < blk[0][i].size(); j++)
        dout << "Block[0][" << i << "][" << j << "].end() = " << blk[0][i][j].getDuration().getEnd()
             << endl;

    auto sl = sp.getSingleLengthAnalytical();

    dout << "sp.getSingleLengthAnalytical() = " << sl << endl;
    REQUIRE(abs(sl - 12.6666) < EPSILON);
  }
}
