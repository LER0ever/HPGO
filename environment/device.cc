#include "device.h"
#include <cassert>

Device::Device() {}

Device::Device(std::vector<int> num_device, std::vector<double> bandwidth) {
  assert(num_device.size() == 2);
  assert(num_device.size() == bandwidth.size());
  this->DeviceName  = "Root Cluster";
  this->Level       = 0;
  this->Rank        = 0;
  this->IsLastLevel = false;
  this->IsOccupied  = false;
  for (auto i = 0; i < num_device[0]; i++) {
    Device s;
    s.Rank        = i;
    s.Level       = 1;
    s.DeviceName  = "Machine #" + std::to_string(i);
    s.IsLastLevel = false;
    s.IsOccupied  = false;
    for (auto j = 0; j < num_device[1]; j++) {
      Device ss;
      ss.Rank = j;
      ss.Level = 2;
      ss.DeviceName = "GPU #" + std::to_string(i);
      ss.IsLastLevel = true;
      ss.IsOccupied = false;
      ss.Memory = 16.0 * 1000000000;
      ss.ComputePower = 1.0;
      s.SubDevices.push_back(ss);
    }
    this->SubDevices.push_back(s);
  }
}
