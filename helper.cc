#include "helper.h"
#include <cassert>

std::vector<std::string> split(const std::string &str,
                               const std::string &delim) {
  std::vector<std::string> tokens;
  size_t                   prev = 0, pos = 0;
  do {
    pos = str.find(delim, prev);
    if (pos == std::string::npos) pos = str.length();
    std::string token = str.substr(prev, pos - prev);
    if (!token.empty()) tokens.push_back(token);
    prev = pos + delim.length();
  } while (pos < str.length() && prev < str.length());
  return tokens;
}

std::vector<double> PythonListToDoubleVector(std::string s) {
  std::vector<double> result;
  if (s[0] == '[') s = s.substr(1, s.length() - 1);
  if (s[s.length() - 1] == ']') s = s.substr(0, s.length() - 1);
  auto vs = split(s, ", ");
  for (auto d : vs) {
    if (d == "None")
      result.push_back(-1);
    else
      result.push_back(atof(d.c_str()));
  }
  assert(vs.size() == result.size());
  return result;
}

std::vector<std::vector<double>> Python2DListToDouble2DVector(std::string s) {
  std::vector<std::vector<double>> result;
  if (s.substr(0, 2) == "[[") s = s.substr(2, s.length() - 2);
  if (s.substr(s.length() - 2, 2) == "]]") s = s.substr(0, s.length() - 2);
  auto vss = split(s, "], [");
  for (auto subs : vss) {
    auto vd = PythonListToDoubleVector(subs);
    result.push_back(vd);
  }
  assert(result.size() == vss.size());
  return result;
}
