#include "model.h"

Layer::Layer() {}

Layer::Layer(int id, double compTime, double activationSize, double paramSize) {
    this->id = id;
    this->compTime = compTime;
    this->activationSize = activationSize;
    this->paramSize = paramSize;
}

Layer::Layer(int id, std::string sType, double compTime, double activationSize, double paramSize) {
    this->id = id;
    this->compTime = compTime;
    this->activationSize = activationSize;
    this->paramSize = paramSize;
    this->sType = sType;
}

Model::Model() {}

Model::Model(int nLayers, std::vector<double> compTime, std::vector<double> activationSize, std::vector<double> paramSize) {
    for (int i=0; i<nLayers; i++) {
        this->Layers.push_back(Layer(i, compTime[i], activationSize[i], paramSize[i]));
    }
}