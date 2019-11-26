#include <HPGO/config.h>
#include <HPGO/environment/network.h>
#include <HPGO/parallelism/split-concat/split-concat.h>

double SplitConcat(Devices d, std::set<int> from, std::set<int> to, double size) {
#ifdef SPLIT_CONCAT_STRICT_CROSS_MACHINE_CHECK
  bool bToCross = d.is_cross_machine(to);
  if (!bToCross) {
    // find all cards from different machines
    std::set<int> cross_machine_cards;
    for (auto s : from) {
      if (d.is_cross_machine(std::set<int>{s}, to)) {
        cross_machine_cards.insert(s);
      }
    }
    if (cross_machine_cards.size() > 0) {
      // assume the part transferred via ethernet far outweighs that with NVLink
      return size / from.size() * cross_machine_cards.size() / B_ETHERNET;
    } else {
      return size / B_NVLINK;
    }
  } else {
    return d.is_cross_machine(from, to) ? (size / B_ETHERNET) : (size / B_NVLINK);
  }
#else
  return d.is_cross_machine(from, to) ? (size / B_ETHERNET) : (size / B_NVLINK);
#endif
  return 0.0;
}
