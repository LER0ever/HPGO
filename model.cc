#include "model.h"

State::State() {}

State::State(int ID, std::string Name, std::string Desc, double CompTime, double ActivationSize,
             double OutputActivationSize, double ParamSize) {
  this->ID                   = ID;
  this->Name                 = Name;
  this->Desc                 = Desc;
  this->CompTime            = CompTime;
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
                                   py::extract<std::string>(state.attr("node_desc")),
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