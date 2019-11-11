#include <HPGO/parallelism/data-parallel/data-parallel.h>
#include <HPGO/environment/network.h>

double DataParallel(Devices d, std::set<int> wids, double size) {
  bool bCross = d.is_cross_machine(wids);
  if (bCross) {
    return size * 2.0 / B_ETHERNET;
  } else {
    return size * 2.0 / B_NVLINK / (wids.size()/2);
  }
  return 0.0;
}
