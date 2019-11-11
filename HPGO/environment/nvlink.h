#ifndef NVLINK_H
#define NVLINK_H

class NVLink {
 public:
  NVLink();
  double AllReduceBandwidth(int num_cards);
  double P2PBandwidth();

 private:
  double baseBandwidth;
};

#endif  // NVLINK_H
