
#pragma once


namespace fiblib
{


template <>
class HPGO_API CTFibonacci<0>
{
public:
    enum {
        value = 0
    };
};

template <>
class HPGO_API CTFibonacci<1>
{
public:
    enum {
        value = 1
    };
};


} // namespace fiblib
