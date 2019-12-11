use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyModule};
use model::model_perf;
use super::ModelImporter;
use input::{torch_graph_py};

pub struct TorchGraphImporter {
}


impl ModelImporter for TorchGraphImporter {
    fn new() -> TorchGraphImporter {
        TorchGraphImporter{}
    }
    fn ImportFrom(&self, filename: &str) -> Option<model_perf::ModelPerf> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let graph = PyModule::from_code(py, torch_graph_py::TORCH_GRAPH_PY, "torch_graph.py", "torch_graph").unwrap();
        let result: (PyObject, PyObject, Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<f64>, Vec<Vec<u32>> ) = graph.call1("prepare", (filename,)).unwrap().extract().unwrap();
        // TODO: no error handling at all
        Some(model_perf::ModelPerf{
            compute_times: result.2,
            activation_sizes: result.3,
            parameter_sizes: result.4,
            output_activation_sizes: result.5,
            all_predecessor_ids: result.6,
        })
    }
}

