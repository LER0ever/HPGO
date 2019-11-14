#ifndef NETWORK_H
#define NETWORK_H

// TODO: hardcode for now
const double GIGA            = 1000000000;
const double B_ETHERNET      = 1 * GIGA;
const double B_ETHERNET_NCCL = 3 * GIGA;
const double B_NVLINK        = 25 * GIGA;

class Network {
 public:
  Network();
  explicit Network(double);
  double        BandwidthSigmoid(double size);
  static double BandwidthSigmoid(double bandwidth, double size);

 private:
  double bandwidth;
};

#endif
