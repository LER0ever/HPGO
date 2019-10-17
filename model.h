#ifndef MODEL_H
#define MODEL_H

#include <string>
#include <vector>
#include "graph.h"

class State {
 public:
  State();
  State(int ID, std::string Name, double CompTime, double ActivationSize,
        double OutputActivationSize, double ParamSize);

  int         ID;
  std::string Name;
  double      CompTime;
  double      ActivationSize;
  double      OutputActivationSize;
  double      ParamSize;
  int         StageID;
};

class Layer {
 public:
  Layer(int ID, double CompTime, double ActivationSize, double OutputActivationSize,
        double ParamSize);

  int         ID;
  std::string Name;
  std::string Desc;
  double      CompTime;
  double      ActivationSize;
  double      OutputActivationSize;
  double      ParamSize;
};

class Model {
 public:
  Model();
  Model(int, pyo);
  void Normalize();
  void SetLayerStats(std::vector<std::vector<double>> compute_times,
                     std::vector<std::vector<double>> activation_sizes,
                     std::vector<std::vector<double>> parameter_sizes,
                     std::vector<double>              output_activation_sizes);

  std::vector<State> States;
  std::vector<Layer> Layers;
  int                GlobalBatchSize;
  int                ProfileBatchSize;
  bool               AllowAsync;
};

#endif  // MODEL_H
