#include "syncpipeline.h"
#include <HPGO/parallelism/data-parallel/data-parallel.h>
#include <HPGO/parallelism/split-concat/split-concat.h>
#include <algorithm>
#include <cassert>

#define DEBUG
#ifdef DEBUG
#include <iostream>
using namespace std;
#endif

SyncPipeline::SyncPipeline() {}

SyncPipeline::SyncPipeline(int mB) { this->mBatch = mB; }

SyncPipeline::SyncPipeline(int mB, std::vector<double> F, std::vector<double> OF,
                           std::vector<double> B, std::vector<double> OB)
    : Pipeline(F, OF, B, OB) {
  this->mBatch = mB;
  this->setInitMB(2 * this->getStage() - 1);
}

double SyncPipeline::getSingleLength() {
  // Prepare the DS
  this->blk.resize(this->mBatch);
  for (int i = 0; i < this->mBatch; i++) {
    this->blk[i].resize(2);
    for (int j = 0; j < 2; j++) this->blk[i][j].resize(this->getInitMB());
  }

  // Arrange the first microbatch
  assert(this->mBatch > 0);
  double curtime = 0.0;
  for (int j = 0; j < this->getInitMB(); j++) {
    if (j % 2 == 0) {  // COMP
      this->blk[0][0][j] = Block(0, j / 2, COMP, FORWARD);
      this->blk[0][0][j].setDuration(Duration(curtime, curtime + F[j / 2]));
      curtime += F[j / 2];
    } else {
      this->blk[0][0][j] = Block(0, j / 2, COMM, NETWORK_UP);
      this->blk[0][0][j].setDuration(
          Duration(curtime, curtime + OF[j / 2] / 100));  // TODO: Network fake
      curtime += OF[j / 2] / 100;
    }
  }
  for (int j = this->getInitMB() - 1; j >= 0; j--) {
    if (j % 2 == 0) {
      this->blk[0][1][j] = Block(0, j / 2, COMP, BACKWARD);
      this->blk[0][1][j].setDuration(Duration(curtime, curtime + B[j / 2]));
      curtime += B[j / 2];
    } else {
      this->blk[0][1][j] = Block(0, j / 2, COMM, NETWORK_DOWN);
      this->blk[0][1][j].setDuration(Duration(curtime, curtime + OB[j / 2] / 100));
      curtime += OB[j / 2] / 100;
    }
  }

  // Arrange microbatch 2 and onwards

  // TODO:

  for (int i = 1; i < mBatch; i++) {
  }

  return 0.0;
}

double SyncPipeline::getSingleLengthAnalytical() {
  // get the max of (forward + backward)
  double maxfb = 0.0, for_bubble = 0.0, back_bubble = 0.0;
  int    maxindex = 0;
  for (int i = 0; i < this->getInitMB(); i++) {
    if (i % 2 == 0) {  // COMP
      maxindex = (F[i / 2] + B[i / 2] >= maxfb) ? i : maxindex;
      maxfb    = std::max(maxfb, F[i / 2] + B[i / 2]);
    } else {  // COMM
      maxindex = (TOF[i / 2] + TOB[i / 2] >= maxfb) ? i : maxindex;
      maxfb    = std::max(maxfb, TOF[i / 2] + TOB[i / 2]);
    }
  }
  // get the bubble for maxfb
  for (int i = 0; i < maxindex; i++) {
    for_bubble += (i % 2 == 0) ? F[i / 2] : TOF[i / 2];
    back_bubble += (i % 2 == 0) ? B[i / 2] : TOB[i / 2];
  }

  return maxfb * this->mBatch + for_bubble + back_bubble;
}

std::vector<std::vector<std::vector<Block>>> SyncPipeline::getBlock() { return this->blk; }

double SyncPipelineSpeedup(Model m, Devices d, int rp, double pipeline_time,
                           std::vector<std::tuple<int, int, int, std::set<int>>> p) {
  auto   compute_times           = m.Meta.compute_times;
  auto   activation_sizes        = m.Meta.activation_sizes;
  auto   parameter_sizes         = m.Meta.parameter_sizes;
  auto   output_activation_sizes = m.Meta.output_activation_sizes;
  double total_compute_times     = compute_times[0][compute_times[0].size() - 1];

  int mBatch = m.GlobalBatchSize / rp / m.MinMicroBatchSize;
#ifdef DEBUG
  cout << "using mBatch = " << mBatch << endl;
#endif

  double block_time = pipeline_time / (double)mBatch;
  double pipeline_length_wout_dp =
      block_time * (mBatch - 1) + total_compute_times / (double)rp / (double)mBatch;
  cout << "block : " << block_time
       << " | total/rp/mbatch : " << total_compute_times / (double)rp / (double)mBatch << endl;

#ifdef DEBUG
  cout << "pipeline_length without DP = " << pipeline_length_wout_dp << endl;
#endif

  double pipeline_length_with_activations = pipeline_length_wout_dp;
  for (int i = 0; i < p.size() - 1; i++) {
    double cut_activations =
        output_activation_sizes[std::get<1>(p[i]) - 1] / (double)rp / (double)mBatch;
    // activation_sizes[std::get<1>(p[i])][std::get<1>(p[i])+1]  / (double)rp / (double)mBatch;
#ifdef DEBUG
    cout << "cut_activations for stage " << i << " ~ " << i + 1 << " = " << cut_activations
         << ", with original value: " << output_activation_sizes[std::get<1>(p[i]) - 1] << endl;
    cout << "time needed for transmission = "
         << SplitConcat(d, std::get<3>(p[i]), std::get<3>(p[i + 1]), cut_activations) << endl;
#endif
    pipeline_length_with_activations +=
        SplitConcat(d, std::get<3>(p[i]), std::get<3>(p[i + 1]), cut_activations);
  }

#ifdef DEBUG
  cout << "pipeline_length after activations = " << pipeline_length_with_activations << endl;
#endif

  double delta = 0;
  for (int i = 0; i < p.size(); i++) {
    double ARTime =
        DataParallel(d, std::get<3>(p[i]), parameter_sizes[std::get<0>(p[i])][std::get<1>(p[i])]);
    if (ARTime > i * block_time) {
      delta = max(ARTime - i * block_time, delta);
    }
  }

#ifdef DEBUG
  if (p.size() == 2) {
    cout << "only have 2 stage, outputing activation size:" << endl;
    double cut_activations         = output_activation_sizes[std::get<1>(p[1]) - 1];
    double profile_cut_activations = cut_activations / m.GlobalBatchSize * m.ProfileBatchSize;
    cout << "profiled at " << profile_cut_activations
         << ", after normalization: " << cut_activations << ", time needed: "
         << SplitConcat(d, std::get<3>(p[0]), std::get<3>(p[1]), cut_activations) << endl;
    cout << "outputing two stage computation times:" << endl;
    for (int i=0; i<2; i++) {
      cout << "Stage " << i << " : " << compute_times[std::get<0>(p[i])][std::get<1>(p[i])] << " / " << std::get<2>(p[i]) << " / " << rp << " = " << compute_times[std::get<0>(p[i])][std::get<1>(p[i])] / std::get<2>(p[i]) / rp << endl;
    }
  }
#endif

#ifdef DEBUG
  cout << "pipeline_length after DP = " << (pipeline_length_with_activations + delta) << endl;
#endif

  return total_compute_times / (pipeline_length_with_activations + delta);
}
