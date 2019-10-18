#include "graph.h"
#include <iostream>
#include <string>

Graph::Graph() { Py_Initialize(); }

struct iterable_converter {
  template <typename Container>
  iterable_converter& from_python() {
    boost::python::converter::registry::push_back(&iterable_converter::convertible,
                                                  &iterable_converter::construct<Container>,
                                                  boost::python::type_id<Container>());

    // Support chaining.
    return *this;
  }

  /// Check if PyObject is iterable.
  static void* convertible(PyObject* object) { return PyObject_GetIter(object) ? object : NULL; }

  template <typename Container>
  static void construct(PyObject*                                                 object,
                        boost::python::converter::rvalue_from_python_stage1_data* data) {
    namespace python = boost::python;
    // Object is a borrowed reference, so create a handle indicting it is
    // borrowed for proper reference counting.
    python::handle<> handle(python::borrowed(object));

    // Obtain a handle to the memory block that the converter has allocated
    // for the C++ type.
    typedef python::converter::rvalue_from_python_storage<Container> storage_type;
    void* storage = reinterpret_cast<storage_type*>(data)->storage.bytes;

    typedef python::stl_input_iterator<typename Container::value_type> iterator;

    // Allocate the C++ type into the converter's memory block, and assign
    // its handle to the converter's convertible variable.  The C++
    // container is populated by passing the begin and end iterators of
    // the python object to the container's constructor.
    new (storage) Container(iterator(python::object(handle)),  // begin
                            iterator());                       // end
    data->convertible = storage;
  }
};

Graph::Graph(std::string filename) {
  Py_Initialize();

  // Initialize the graph gr object
  try {
    auto m_io  = py::import("io");
    auto m_sys = py::import("sys");
    m_sys.attr("path").attr("append")("..");
    mUtils                     = py::import("utils");
    pyo retPrep                = mUtils.attr("prepare")(filename, true);
    py_gr                      = retPrep[0];
    py_states                  = retPrep[1];
    py_compute_times           = retPrep[2];
    py_activation_sizes        = retPrep[3];
    py_parameter_sizes         = retPrep[4];
    py_output_activation_sizes = retPrep[5];
    py_all_predecessor_ids     = retPrep[6];
  } catch (...) {
    PyErr_Print();
    PyErr_Clear();
  }

  // Conversion to C++
  try {
    iterable_converter()
        .from_python<std::vector<double>>()
        .from_python<std::vector<std::vector<double>>>()
        .from_python<std::vector<std::vector<int>>>();
    compute_times           = py::extract<std::vector<std::vector<double>>>(py_compute_times);
    activation_sizes        = py::extract<std::vector<std::vector<double>>>(py_activation_sizes);
    parameter_sizes         = py::extract<std::vector<std::vector<double>>>(py_parameter_sizes);
    output_activation_sizes = py::extract<std::vector<double>>(py_output_activation_sizes);
    for (auto i = 0; i < py::len(py_all_predecessor_ids); i++) {
      std::vector<int> vi;
      for (auto j = 0; j < py::len(py_all_predecessor_ids[i]); j++) {
        vi.push_back(py::extract<int>(py_all_predecessor_ids[i][j]));
      }
      all_predecessor_ids.push_back(vi);
    }
  } catch (...) {
    PyErr_Print();
    PyErr_Clear();
  }
}

pyo  Graph::getStates() { return py_states; }
pyo  Graph::getGR() { return py_gr; }
void Graph::update_stage_id(int end, int stage_id) {
  try {
    mUtils.attr("update_stage_id")(py_gr, py_states, end, stage_id);
  } catch (...) {
    PyErr_Print();
    PyErr_Clear();
  }
}
