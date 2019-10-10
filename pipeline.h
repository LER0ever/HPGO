#ifndef PIPELINE_H
#define PIPELINE_H

#include <vector>

class Pipeline {
 public:
  // Functions

  // Initialization
  Pipeline();
  Pipeline(std::vector<double>, std::vector<double>, std::vector<double>,
           std::vector<double>);
  void setStage(int);
  void setInitMB(int);

  int getStage();
  int getInitMB();

  std::vector<double> F, B, OF, OB, TOF, TOB;

  // Helper
 private:
  bool bSync;
  int  nStage;
  int  initMB;  // initial micro batches
};

#endif  // PIPELINE_H
