#include "HPGO.h"
#include <boost/python.hpp>

TypeA compute_partitioning(std::vector<double> compute_times,
                           std::vector<double> activation_sizes,
                           std::vector<double> parameter_sizes,
                           std::vector<double> output_activation_sizes,
                           std::vector<int>    all_predecessor_ids,
                           int num_machines, int num_machines_within_machine,
                           std::vector<double> bandwidth, bool final_level) {
  TypeA A;

  return A;
}

//#define BOOST_PYTHON_MAX_ARITY 24
// BOOST_PYTHON_MODULE(HPGO)
//{
//    using namespace boost::python;
//    def("compute_partitioning", compute_partitioning);
//}
