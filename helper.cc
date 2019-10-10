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
    /*if (!token.empty()) */ tokens.push_back(token);
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

std::vector<int> PythonListToIntVector(std::string s) {
  std::vector<int> result;
  if (s[0] == '[') s = s.substr(1, s.length() - 1);
  if (s[s.length() - 1] == ']') s = s.substr(0, s.length() - 1);
  auto vs = split(s, ", ");
  for (auto d : vs) {
    if (d == "None")
      result.push_back(-1);
    else if (d == "") {
    } else
      result.push_back(atoi(d.c_str()));
  }

  assert(vs.size() == result.size() || (vs.size() == 1 && result.size() == 0));
  return result;
}

std::vector<std::vector<int>> Python2DListToInt2DVector(std::string s) {
  std::vector<std::vector<int>> result;
  if (s[0] == '[') s = s.substr(1, s.length() - 1);
  if (s[s.length() - 1] == ']') s = s.substr(0, s.length() - 1);

  std::vector<size_t> lbrace, rbrace;
  auto                foundl = s.find("[");
  auto                foundr = s.find("]");
  while (foundl != std::string::npos) {
    lbrace.push_back(foundl);
    foundl = s.find("[", foundl + 1);
  }
  while (foundr != std::string::npos) {
    rbrace.push_back(foundr);
    foundr = s.find("]", foundr + 1);
  }
  assert(lbrace.size() == rbrace.size());
  for (int i = 0; i < lbrace.size(); i++) {
    assert(lbrace[i] < rbrace[i]);
    auto subs = s.substr(lbrace[i], rbrace[i] - lbrace[i]);
    auto vd   = PythonListToIntVector(subs);
    result.push_back(vd);
  }
  //  auto vss = split(s, "], [");
  //  for (auto subs : vss) {
  //    auto vd = PythonListToIntVector(subs);
  //    result.push_back(vd);
  //  }
  //  assert(result.size() == vss.size());
  return result;
}
