#ifndef NETWORK_H
#define NETWORK_H

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