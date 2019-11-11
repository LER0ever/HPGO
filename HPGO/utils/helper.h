#ifndef HELPER_H
#define HELPER_H

#include <string>
#include <vector>
#include <HPGO/HPGO_api.h>

#define frs(x,a,b) for(size_t x=a; x<b; x++)
#define fri(x,a,b) for(int x=a; x<b; x++)

std::vector<std::string>         split(const std::string &str,
                                       const std::string &delim);
HPGO_API std::vector<double>              PythonListToDoubleVector(std::string s);
HPGO_API std::vector<std::vector<double>> Python2DListToDouble2DVector(std::string s);
HPGO_API std::vector<int>                 PythonListToIntVector(std::string s);
HPGO_API std::vector<std::vector<int>>    Python2DListToInt2DVector(std::string s);

#endif  // HELPER_H
