#ifndef SPLIT_CONCAT_H
#define SPLIT_CONCAT_H

#include <HPGO/HPGO_api.h>
#include <HPGO/environment/device.h>
#include <vector>
#include <set>

HPGO_API double SplitConcat(Devices d, std::set<int> from, std::set<int> to, double size);

#endif  // SPLIT_CONCAT_H
