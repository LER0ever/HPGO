#include "model.h"

State::State() {}

State::State(int ID, std::string Name, double CompTime, double ActivationSize,
             double OutputActivationSize, double ParamSize) {
  this->ID                   = ID;
  this->Name                 = Name;
  this->CompTime             = CompTime;
  this->ActivationSize       = ActivationSize;
  this->OutputActivationSize = OutputActivationSize;
  this->ParamSize            = ParamSize;
}

Model::Model() {}

Model::Model(int globalBatchSize, pyo pyStates) {
  this->GlobalBatchSize = globalBatchSize;
  try {
    for (int i = 0; i < py::len(pyStates); i++) {
      pyo state = pyStates[i];
      this->States.push_back(State(i, py::extract<std::string>(state.attr("node_id")),
                                   py::extract<double>(state.attr("compute_time")),
                                   py::extract<double>(state.attr("activation_size")),
                                   py::extract<double>(state.attr("output_activation_size")),
                                   py::extract<double>(state.attr("parameter_size"))));
    }
  } catch (...) {
    PyErr_Print();
    PyErr_Clear();
  }
}

void Model::Normalize() {
  double factor = (double)this->GlobalBatchSize / (double)this->ProfileBatchSize;
  for (auto s : this->States) {
    s.CompTime *= factor;
    s.ActivationSize *= factor;
    s.OutputActivationSize *= factor;
  }
}

void Model::SetLayerStats(std::vector<std::vector<double>> compute_times,
                          std::vector<std::vector<double>> activation_sizes,
                          std::vector<std::vector<double>> parameter_sizes,
                          std::vector<double>              output_activation_sizes) {
  for (int i = 0; i < compute_times.size() - 1; i++) {
    this->Layers.push_back(Layer(i, compute_times[i][i], activation_sizes[i][i],
                                 parameter_sizes[i][i], output_activation_sizes[i]));
  }
}

Layer::Layer(int ID, double CompTime, double ActivationSize, double OutputActivationSize,
             double ParamSize) {
  this->ID                   = ID;
  this->CompTime             = CompTime;
  this->ActivationSize       = ActivationSize;
  this->OutputActivationSize = OutputActivationSize;
  this->ParamSize            = ParamSize;
}