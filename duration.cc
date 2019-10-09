#include "duration.h"
#include <algorithm>

double Timestamp::operator-(Timestamp t)
{
    return this->t - t.t;
}

double Timestamp::getTime()
{
    return this->t;
}

Duration::Duration(){}

Duration::Duration(Timestamp t1, Timestamp t2)
{
    this->s = std::min(t1.getTime(), t2.getTime());
    this->t = std::max(t1.getTime(), t2.getTime());
    this->d = this->t - this->s;
}

Duration::Duration(double t1, double t2)
{
    this->s = std::min(t1, t2);
    this->t = std::max(t1, t2);
    this->d = this->t - this->s;
}

double Duration::getDuration() {return this->d;}
double Duration::getStart() { return this->s; }
double Duration::getEnd() { return this->t; }
