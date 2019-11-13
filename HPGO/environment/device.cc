#include "device.h"
#include <cassert>
#include <map>

Device::Device() {}

Device::Device(std::vector<int> num_device, std::vector<double> bandwidth) {
  // FIXME: this function is hardcoded
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
      ss.Rank         = j;
      ss.Level        = 2;
      ss.DeviceName   = "GPU #" + std::to_string(i);
      ss.IsLastLevel  = true;
      ss.IsOccupied   = false;
      ss.Memory       = 16.0 * 1000000000;
      ss.ComputePower = 1.0;
      s.SubDevices.push_back(ss);
    }
    this->SubDevices.push_back(s);
  }
}

Devices::Devices(int n, std::vector<int> sep) {
  this->seps = sep;
  for (auto i = 0; i < n; i++) dbs.push_back(false);
  this->M = sep.size();
  this->G = n;
}

std::vector<BNRet> Devices::bitnext(int need) { return bitnext(dbs, need); }

std::vector<BNRet> Devices::bitnext(std::vector<bool> bs, int need) {
  int                               start = 0, left_total = 0;
  std::vector<std::pair<int, bool>> machines;  // slots left, fresh
  std::vector<BNRet>                res;
  std::map<std::set<int>, BNRet>    exist;
  for (auto i = 0; i < this->seps.size(); i++) {
    bool fresh = true;
    int  left  = 0;
    for (auto j = start; j < this->seps[i]; j++) {
      if (bs[j] != 0) {
        fresh = false;
      } else {
        left++;
      }
    }
    machines.push_back(std::pair<int, bool>(left, fresh));
    left_total += left;
    start = this->seps[i];
  }

  assert(left_total >= need);

  // FF
  {
    // std::pair<std::vector<bool>, std::vector<int>> t_FF;
    BNRet t_FF;
    std::get<0>(t_FF) = 0;
    std::get<1>(t_FF) = bs;
    int n_FF          = need;
    for (int i = 0; i < machines.size(); i++) {
      if (machines[i].second == true) {  // only if fresh = true
        if (machines[i].first >= n_FF) {
          auto j = (i == 0 ? 0 : seps[i - 1]);
          while (n_FF) {
            std::get<1>(t_FF)[j] = true;
            std::get<2>(t_FF).insert(j);
            j++;
            n_FF--;
          }
        } else {
          // n_AF -= machines[i].first;
          auto j = (i == 0 ? 0 : seps[i - 1]);
          while (std::get<1>(t_FF)[j] == true) j++;
          while (j < seps[i]) {
            std::get<1>(t_FF)[j] = true;
            std::get<2>(t_FF).insert(j);
            j++;
            n_FF--;
          }
        }
      }
    }
    if (n_FF == 0) {
      exist.insert_or_assign(std::get<2>(t_FF), t_FF);
      // res.push_back(t_FF);  // only valid if n_FF down to 0
    }
  }

  // AF
  {
    BNRet t_AF;
    std::get<0>(t_AF) = 1;
    std::get<1>(t_AF) = bs;
    int n_AF          = need;
    for (int i = 0; i < machines.size(); i++) {
      if (machines[i].second != true) {
        if (machines[i].first >= n_AF) {
          auto j = (i == 0 ? 0 : seps[i - 1]);
          while (std::get<1>(t_AF)[j] == true) j++;
          while (n_AF) {
            std::get<1>(t_AF)[j] = true;
            std::get<2>(t_AF).insert(j);
            j++;
            n_AF--;
          }
        } else {
          // n_AF -= machines[i].first;
          auto j = (i == 0 ? 0 : seps[i - 1]);
          while (std::get<1>(t_AF)[j] == true) j++;
          while (j < seps[i]) {
            std::get<1>(t_AF)[j] = true;
            std::get<2>(t_AF).insert(j);
            j++;
            n_AF--;
          }
          // machines[i].first = 0;
        }
      }
    }
    if (n_AF == 0) {
      exist.insert_or_assign(std::get<2>(t_AF), t_AF);
    }  // res.push_back(t_AF);
  }

  // SF
  // This block changes Machines, should be the last one
  {
    BNRet t_SF;
    std::get<0>(t_SF) = 2;
    std::get<1>(t_SF) = bs;
    int n_SF          = need;
    while (n_SF) {
      for (int i = 0; i < machines.size(); i++) {
        if (machines[i].first > 0 && n_SF > 0) {
          auto j = (i == 0 ? 0 : seps[i - 1]);
          while (std::get<1>(t_SF)[j] == true) j++;
          std::get<1>(t_SF)[j] = true;
          std::get<2>(t_SF).insert(j);
          n_SF--;
          machines[i].first--;
        }
      }
    }
    if (n_SF == 0) {
      exist.insert_or_assign(std::get<2>(t_SF), t_SF);
    }  // res.push_back(t_SF);
  }

  for (auto it = exist.begin(); it != exist.end(); ++it) {
    res.push_back(it->second);
  }

  return res;
}

std::vector<std::vector<BNRet>> Devices::bitnext(std::vector<bool> bs, int need, int replica)
{
  auto SingleBN = this->bitnext(bs, need);
  std::vector<std::vector<BNRet>> res;
  if (replica == 1) {
    for (auto sbn : SingleBN) {
      res.push_back(std::vector<BNRet>{sbn});
    }
    return res;
  }
  for (auto sbn : SingleBN) {
    auto cur_bs = std::get<1>(sbn);
    auto cur_wids = std::get<2>(sbn);
    auto subres = bitnext(cur_bs, need, replica-1);
    for (auto sr : subres) {
      auto newsr = sr;
      newsr.push_back(sbn);
      res.push_back(newsr);
    }
  }
  return res;
}

bool Devices::is_cross_machine(std::set<int> from, std::set<int> to) {
  // FIXME: this is a stupid O(N^3) traversal
  // NOTE: fixed using a heuristic, check later
  int min_from = *std::min_element(from.begin(), from.end()),
       max_from = *std::max_element(from.begin(), from.end()),
       min_to   = *std::min_element(to.begin(), to.end()),
       max_to   = *std::max_element(to.begin(), to.end());
  int a = std::min(min_from, min_to), b = std::max(max_from, max_to);

  int l = 0;
  for (auto r : this->seps) {
    if (l <= a && a <= r && l <= b && b <= r) return false;
    l = r;
  }

  return true;
}

bool Devices::is_cross_machine(std::set<int> wids) {
  int min_id = *std::min_element(wids.begin(), wids.end()),
    max_id = *std::max_element(wids.begin(), wids.end());
  int l=0;
  for (auto r: this->seps) {
    if (l <= min_id && min_id < r && l <= max_id && max_id < r) return false;
    l = r;
  }

  return true;
}
