#ifndef HPGO_H
#define HPGO_H

#include <tuple>
#include <vector>
#include <HPGO/HPGO_api.h>

using triple = std::tuple<double, std::pair<int, int>, double>;
using TypeA  = std::vector<std::vector<std::vector<triple> > >;


// Public API, WIP

class HPGO_API HPGO {
 public:

 private:
};

#endif  // HPGO_H
