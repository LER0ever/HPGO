#include <HPGO/model/model.h>
#include <HPGO/utils/helper.h>

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

Model::Model(int globalBatchSize, int profileBatchSize, int minMicroBatchSize, Graph g) {
  this->GlobalBatchSize   = globalBatchSize;
  this->ProfileBatchSize  = profileBatchSize;
  this->MinMicroBatchSize = minMicroBatchSize;  // FIXME: Hardcoded
  pyo pyStates            = g.getStates();
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
  this->Meta.compute_times           = g.compute_times;
  this->Meta.activation_sizes        = g.activation_sizes;
  this->Meta.parameter_sizes         = g.parameter_sizes;
  this->Meta.output_activation_sizes = g.output_activation_sizes;
  this->Meta.all_predecessor_ids     = g.all_predecessor_ids;

  // normalize to GBS
  this->normalize();
  this->setLayerStats(this->Meta.compute_times, this->Meta.activation_sizes, this->Meta.parameter_sizes, this->Meta.output_activation_sizes);
}

void Model::normalize() {
  double factor = (double)this->GlobalBatchSize / (double)this->ProfileBatchSize;
  for (auto s : this->States) {
    s.CompTime *= factor;
    s.ActivationSize *= factor;
    s.OutputActivationSize *= factor;
  }
  frs(i, 0, this->Meta.compute_times.size()) {
    frs(j, 0, this->Meta.compute_times[i].size()) {
      this->Meta.compute_times[i][j] *= factor;
    }
  }
  frs(i, 0, this->Meta.activation_sizes.size()) {
    frs(j, 0, this->Meta.activation_sizes[i].size()) {
      this->Meta.activation_sizes[i][j] *= factor;
    }
  }
  frs(i, 0, this->Meta.output_activation_sizes.size()) {
      this->Meta.output_activation_sizes[i] *= factor;
  }
}

void Model::setLayerStats(std::vector<std::vector<double>> compute_times,
                          std::vector<std::vector<double>> activation_sizes,
                          std::vector<std::vector<double>> parameter_sizes,
                          std::vector<double>              output_activation_sizes) {
  for (int i = 0; i < compute_times[0].size(); i++) {
    this->Layers.push_back(Layer(i, compute_times[i][i], activation_sizes[i][i],
                                 parameter_sizes[i][i], output_activation_sizes[i]));
  }
}

Layer::Layer(int ID, double CompTime, double ActivationSize, double ParamSize,
             double OutputActivationSize) {
  this->ID                   = ID;
  this->CompTime             = CompTime;
  this->ActivationSize       = ActivationSize;
  this->OutputActivationSize = OutputActivationSize;
  this->ParamSize            = ParamSize;
}
