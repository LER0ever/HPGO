#include "syncpipeline.h"
#include <algorithm>
#include <iostream>

SyncPipeline::SyncPipeline()
{
}

SyncPipeline::SyncPipeline(int mB)
{
    this->mBatch = mB;
}

SyncPipeline::SyncPipeline(int mB,
                           std::vector<double> F,
                           std::vector<double> OF,
                           std::vector<double> B,
                           std::vector<double> OB)
    :Pipeline (F, OF, B, OB)
{
    this->mBatch = mB;
    this->setInitMB(2*this->getStage()-1);
}

double SyncPipeline::getSingleLength()
{
    // Prepare the DS
    this->blk.resize(this->mBatch);
    for (int i=0; i<this->mBatch; i++){
        this->blk[i].resize(2);
        for (int j=0; j<2; j++)
            this->blk[i][j].resize(this->getInitMB());
    }


    // Arrange the first microbatch
    assert(this->mBatch > 0);
    double curtime = 0.0;
    for (int j=0; j<this->getInitMB(); j++) {
        if (j % 2 == 0 ) { // COMP
            this->blk[0][0][j] = Block(0, j / 2, COMP, FORWARD);
            this->blk[0][0][j].setDuration(
                        Duration(curtime, curtime + F[j / 2])
                    );
            curtime += F[j / 2];
        } else {
            this->blk[0][0][j] = Block(0, j / 2, COMM, NETWORK_UP);
            this->blk[0][0][j].setDuration(
                        Duration(curtime, curtime + OF[j/2] / 100)
                    ); //TODO: Network fake
            curtime += OF[j/2] / 100;
        }
    }
    for (int j=this->getInitMB() - 1; j>=0; j--) {
        if (j % 2 == 0) {
            this->blk[0][1][j] = Block(0, j / 2, COMP, BACKWARD);
            this->blk[0][1][j].setDuration(
                        Duration(curtime, curtime + B[j / 2])
                    );
            curtime += B[j / 2];
        } else {
            this->blk[0][1][j] = Block(0, j / 2, COMM, NETWORK_DOWN);
            this->blk[0][1][j].setDuration(
                        Duration(curtime, curtime + OB[j / 2] / 100)
                    );
            curtime += OB[j / 2] / 100;
        }
    }

    // Arrange microbatch 2 and onwards

    // TODO:

    for (int i=1; i<mBatch; i++) {

    }

    return 0.0;
}

double SyncPipeline::getSingleLengthAnalytical()
{
    // get the max of (forward + backward)
    double maxfb = 0.0, for_bubble = 0.0, back_bubble = 0.0;
    int maxindex = 0;
    for (int i=0; i<this->getInitMB(); i++) {
        if (i % 2 == 0) { // COMP
            maxindex = (F[i/2] + B[i/2] >= maxfb) ? i : maxindex;
            maxfb = std::max(maxfb, F[i/2] + B[i/2]);
        } else { // COMM
            maxindex = (TOF[i/2] + TOB[i/2] >= maxfb) ? i : maxindex;
            maxfb = std::max(maxfb, TOF[i/2] + TOB[i/2]);
        }
    }
    // get the bubble for maxfb
    for (int i=0; i<maxindex; i++) {
        for_bubble += (i % 2 == 0) ? F[i/2] : TOF[i/2];
        back_bubble += (i % 2 == 0) ? B[i/2] : TOB[i/2];
    }

    return maxfb * this->mBatch + for_bubble + back_bubble;
}

std::vector<std::vector<std::vector<Block> > > SyncPipeline::getBlock() {
    return this->blk;
}
