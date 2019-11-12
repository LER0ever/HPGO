#ifndef SYNCPIPELINE_H
#define SYNCPIPELINE_H

#include <vector>
#include <HPGO/HPGO_api.h>
#include <HPGO/parallelism/pipeline/block.h>
#include <HPGO/parallelism/pipeline/duration.h>
#include <HPGO/parallelism/pipeline/pipeline.h>

class HPGO_API SyncPipeline : public Pipeline {
 public:
  SyncPipeline();
  SyncPipeline(int);
  SyncPipeline(int, std::vector<double>, std::vector<double>,
               std::vector<double>, std::vector<double>);
  double                                         getSingleLength();
  double                                         getSingleLengthAnalytical();
  std::vector<std::vector<std::vector<Block> > > getBlock();

 private:
  int mBatch;
  std::vector<std::vector<std::vector<Block> > >
      blk;  // b[i][0/1][j]: i-th micro batch, 0 for, 1 back, j-th stage
};

/* HPGO_API SyncPipelineLength(TA A, int rp); */

#endif  // SYNCPIPELINE_H
