#ifndef DEVICE_H
#define DEVICE_H

#include <HPGO/HPGO_api.h>
#include <set>
#include <string>
#include <vector>

using BNRet = std::tuple<int, std::vector<bool>, std::set<int>>;

struct HPGO_API Device {
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

struct HPGO_API Devices {
  Devices(){};
  Devices(int, std::vector<int>);
  int                M;  // num of machines
  int                G;  // num of cards
  std::vector<bool>  dbs;
  std::vector<int>   seps;
  std::vector<BNRet> bitnext(int);
  std::vector<BNRet> bitnext(std::vector<bool>, int);
  std::vector<std::vector<BNRet>> bitnext(std::vector<bool>, int, int);
  std::vector<BNRet> bnmerge(std::vector<std::vector<BNRet>>);
  bool is_cross_machine(std::set<int>, std::set<int>);
  bool is_cross_machine(std::set<int>);
};

#endif  // DEVICE_H
