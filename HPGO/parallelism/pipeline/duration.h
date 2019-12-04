#ifndef TIME_H
#define TIME_H

#include <HPGO/HPGO_api.h>

class Timestamp {
 public:
  // Functions
  double operator-(Timestamp t);
  double getTime();

  // Initialization
  Timestamp();
  Timestamp(double);
  Timestamp(long);

 private:
  double t;
};

class HPGO_API Duration {
 public:
  // Functions
  double getStart();
  double getEnd();
  double getDuration();

  // Initialization
  Duration();
  Duration(Timestamp, Timestamp);
  Duration(double, double);

 private:
  double s;  // start
  double t;  // end
  double d;  // duration
};

#endif  // TIME_H