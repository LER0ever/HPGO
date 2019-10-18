#include "network.h"
#include <cmath>

Network::Network() { this->bandwidth = 3.0 * 1000000000; }

Network::Network(double bandwidth) { this->bandwidth = bandwidth; }

// apply the network S curve to compensate small package size
double Network::BandwidthSigmoid(double size) {
  return this->bandwidth / (1 + exp(-4 * size / 10000 + 4));
}

double Network::BandwidthSigmoid(double bandwidth, double size) {
  return bandwidth / (1 + exp(-4 * size / 10000 + 4));
}
