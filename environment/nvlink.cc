#include "nvlink.h"

NVLink::NVLink() { this->baseBandwidth = 25.0 * 1000000000; }

double NVLink::AllReduceBandwidth(int num_cards) { return this->baseBandwidth * (num_cards / 2); }
