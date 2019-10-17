#ifndef DEVICE_H
#define DEVICE_H

#include <vector>

class DeviceLevel {
 public:
  int                      Level;
  double                   Bandwidth;  // inter device bandwidth within the same level, bytes / sec
  std::vector<DeviceLevel> SubLevels;
  bool                     IsLastLevel;
  Device                   Device;  // for the last level
};

class Device {
 public:
  double      ComputePower;
  double      Memory;
  std::string DeviceName;
  int         LocalRank;
  bool        IsOccupied;
};

#endif  // DEVICE_H