#ifndef DEVICE_H
#define DEVICE_H

#include <string>
#include <vector>

struct Device {
  Device();
  Device(std::vector<int>, std::vector<double>);
  std::vector<Device> SubDevices;
  int                 Level;
  double              ComputePower;
  double              Memory;
  std::string         DeviceName;
  int                 Rank;
  bool                IsOccupied;
  bool                IsLastLevel;
};

#endif  // DEVICE_H