#include <boost/python.hpp>

char const* greet()
{
    return "hello, world";
}


BOOST_PYTHON_MODULE(test_python)
{
    using namespace boost::python;
    def("greet", greet);
}

