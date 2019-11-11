#ifndef MODEL_H
#define MODEL_H

#include <HPGO/HPGO_api.h>
#include <HPGO/input/graph.h>
#include <string>
#include <vector>

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

struct Metadata {
  std::vector<std::vector<double>> compute_times;
  std::vector<std::vector<double>> activation_sizes;
  std::vector<std::vector<double>> parameter_sizes;
  std::vector<double>              output_activation_sizes;
  std::vector<std::vector<int>>    all_predecessor_ids;
};

class HPGO_API Model {
 public:
  Model();
  Model(int, int, Graph);

  std::vector<State> States;
  std::vector<Layer> Layers;
  Metadata           Meta;
  int                GlobalBatchSize;
  int                ProfileBatchSize;
  int                MinMicroBatchSize;
  bool               AllowAsync;

 private:
  void normalize();
  void setLayerStats(std::vector<std::vector<double>> compute_times,
                     std::vector<std::vector<double>> activation_sizes,
                     std::vector<std::vector<double>> parameter_sizes,
                     std::vector<double>              output_activation_sizes);
};

#endif  // MODEL_H
