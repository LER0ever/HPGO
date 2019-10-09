#ifndef MODEL_H
#define MODEL_H

#include <vector>
#include <string>

class Layer {
public:
    Layer();
    Layer(int, double, double, double);
    Layer(int, std::string, double, double, double);
private:
    int id;
    std::string sType;
    double compTime;
    double activationSize;
    double outputActivationSize;
    double paramSize;
};

class Model {
public:
    Model();
    Model(int, std::vector<double>, std::vector<double>, std::vector<double>);
private:
    std::vector<Layer> Layers;
};

#endif // MODEL_H