#include <HPGO/environment/network.h>
#include <HPGO/parallelism/split-concat/split-concat.h>

double SplitConcat(Devices d, std::set<int> from, std::set<int> to, double size) {
  bool bCross = d.is_cross_machine(from, to);
  // TODO: no network normalization or NVLINK compensation whatsoever, for now.
  if (bCross) {
    return size / B_ETHERNET /
           3;  // NOTE: /3 for network p2p reaching top speed of 1.2G under 3G ethernet
  } else {
    return size / B_NVLINK;
  }
  return 0.0;
}
