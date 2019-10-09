#include <iostream>
#include <cmath>
#include "../syncpipeline.h"
#include "../orchestration.h"

#include "test_config.h"
#include "catch.hpp"
using namespace std;

TEST_CASE( "Pipeline Efficiency Calculation Test", "[PEC]" ) {
    SECTION("4 stage pipeline with F:B=1:2, mBatch = 7") {
        vector<double> F(4, 1);
        vector<double> B(4, 2);
        vector<double> OF(4, 33.333333);
        vector<double> OB(4, 33.333333);
        auto sp = SyncPipeline(7, F, OF, B, OB);
        sp.getSingleLength();
        everMsg("getSingleLength() done");
        vector<vector<vector<Block> > > blk = sp.getBlock();
        for (int i=0; i<blk[0].size(); i++)
            for (int j=0; j<blk[0][i].size(); j++)
                cout << "Block[0][" << i << "][" << j << "].end() = " << blk[0][i][j].getDuration().getEnd() << endl;

        auto sl = sp.getSingleLengthAnalytical();

        cout << "sp.getSingleLengthAnalytical() = " << sl << endl;
        REQUIRE(abs(sl - 32.0) < 0.01);
    }

    SECTION("2 stage pipeline with F:B=1:2, mBatch = 3") {
        vector<double> F(2, 1);
        vector<double> B(2, 2);
        vector<double> OF(2, 33.333333);
        vector<double> OB(2, 33.333333);
        auto sp = SyncPipeline(3, F, OF, B, OB);
        sp.getSingleLength();
        cout << "getSingleLength() done" << endl;
        vector<vector<vector<Block> > > blk = sp.getBlock();
        for (int i=0; i<blk[0].size(); i++)
            for (int j=0; j<blk[0][i].size(); j++)
                cout << "Block[0][" << i << "][" << j << "].end() = " << blk[0][i][j].getDuration().getEnd() << endl;

        auto sl = sp.getSingleLengthAnalytical();

        cout << "sp.getSingleLengthAnalytical() = " << sl << endl;
        REQUIRE(abs(sl - 12.6666) < 0.01);
    }

}

