#include <HPGO/environment/nvlink.h>

NVLink::NVLink() { this->baseBandwidth = 25.0 * 1000000000; }

double NVLink::AllReduceBandwidth(int num_cards) { return this->baseBandwidth * (num_cards / 2); }
double NVLink::P2PBandwidth() { return this->baseBandwidth; }
