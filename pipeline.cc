#include "pipeline.h"
#include <cassert>

Pipeline::Pipeline() { this->bSync = true; }

Pipeline::Pipeline(std::vector<double> F, std::vector<double> OF,
                   std::vector<double> B, std::vector<double> OB) {
  this->F  = F;
  this->OF = OF;
  this->B  = B;
  this->OB = OB;
  this->TOF.resize(OF.size());
  this->TOB.resize(OB.size());
  for (int i = 0; i < OF.size(); i++) {
    this->TOF[i] = OF[i] / 100;
    this->TOB[i] = OB[i] / 100;
  }
  assert(F.size() == B.size());
  this->nStage = F.size();
}

void Pipeline::setStage(int nS) { this->nStage = nS; }

void Pipeline::setInitMB(int mb) { this->initMB = mb; }

int Pipeline::getStage() { return this->nStage; }
int Pipeline::getInitMB() { return this->initMB; }
