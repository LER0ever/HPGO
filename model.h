#ifndef MODEL_H
#define MODEL_H

#include <string>
#include <vector>
#include "graph.h"

class State {
 public:
  State();
  State(int ID, std::string Name, std::string Desc, double CompTime, double ActivationSize,
        double OutputActivationSize, double ParamSize);

  int         ID;
  std::string Name;
  std::string Desc;
  std::string Type;  // UNUSED
  double      CompTime;
  double      ForwardCompTime;   // UNUSED
  double      BackwardCompTime;  // UNUSED
  double      ActivationSize;
  double      OutputActivationSize;
  double      ParamSize;
  int         StageID;
  int         depth;
  int         height;
};

class Model {
 public:
  Model();
  Model(int, pyo);

  std::vector<State> States;
  int                GlobalBatchSize;
  int                ProfileBatchSize;
  bool               AllowAsync;
};

#endif  // MODEL_H
