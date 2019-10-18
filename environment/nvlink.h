#ifndef NVLINK_H
#define NVLINK_H

class NVLink {
 public:
  NVLink();
  double AllReduceBandwidth(int num_cards);

 private:
  double baseBandwidth;
};

#endif  // NVLINK_H