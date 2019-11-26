#include <HPGO/environment/network.h>
#include <HPGO/parallelism/data-parallel/data-parallel.h>

double DataParallel(Devices d, std::set<int> wids, double size) {
  bool   bCross = d.is_cross_machine(wids);
  double factor = (double)(wids.size() - 1) / (double)wids.size();
  if (wids.size() <= 1) return 0.0;
  if (bCross) {
    return size * 2.0 * factor / B_ETHERNET_NCCL;
  } else {
    return size * 2.0 * factor / B_NVLINK / (wids.size() / 2);
  }
  return 0.0;
}

double DPSpeedup(Devices d, double compute, double allreduce) {
  int    num_machines          = d.G;
  double single_machine_time   = compute;
  double dp_multi_machine_time = compute / (double)num_machines + allreduce;

  return single_machine_time / dp_multi_machine_time;
}

double DPSpeedupWeak(Devices d, double compute, double allreduce) {
  int    num_machines = d.G;
  double efficiency   = compute / (compute + allreduce);
  return (double)num_machines * efficiency;
}