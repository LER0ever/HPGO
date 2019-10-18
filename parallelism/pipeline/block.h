#ifndef BLOCK_H
#define BLOCK_H

#include "duration.h"

const int COMP = 0;
const int COMM = 1;

const int FORWARD      = 0;
const int BACKWARD     = 1;
const int NETWORK_UP   = 0;
const int NETWORK_DOWN = 1;

class Block {
 public:
  Block();
  Block(int, int, int, int);

  void     setDuration(Duration d);
  Duration getDuration();

  int getBatch();
  int getStage();
  int getType();
  int getDir();

 private:
  int      nBatch;  // micro batch number
  int      nStage;  // stage number
  int      nType;   // COMP vs COMM
  int      nDir;    // Direction Forward vs Backward
  Duration dur;
};

#endif  // BLOCK_H
