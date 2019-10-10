#ifndef HELPER_H
#define HELPER_H

#include <string>
#include <vector>

std::vector<std::string>         split(const std::string &str,
                                       const std::string &delim);
std::vector<double>              PythonListToDoubleVector(std::string s);
std::vector<std::vector<double>> Python2DListToDouble2DVector(std::string s);

#endif  // HELPER_H
