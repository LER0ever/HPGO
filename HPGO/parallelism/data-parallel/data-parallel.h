#ifndef DP_H
#define DP_H

#include <HPGO/HPGO_api.h>
#include <HPGO/environment/device.h>
#include <vector>
#include <set>

HPGO_API double DataParallel(Devices d, std::set<int> wids, double size);

#endif
