use crate::environment::*;
use crate::input::*;
use crate::layerwise::model::model::Model;
use crate::layerwise::orchestration::*;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Conductor {
    #[pyo3(get)]
    pub model: Model,
    #[pyo3(get)]
    pub filename: String,
}

#[pymethods]
impl Conductor {
    /// Construct a new Conductor for working with HLO/MLIR model planning
    #[new]
    pub fn new() -> Self {
        Conductor {
            model: Model::default(),
            filename: String::default(),
        }
    }

    pub fn import_from(&mut self, filename: String, pbs: u32, gbs: u32) -> PyResult<()> {
        self.filename = filename;
        let tgi: torch_graph::TorchGraphImporter = LayerwiseModelImporter::new();
        let result = tgi.ImportFrom(&self.filename);
        let (perf, states) = (result.0.unwrap(), result.1.unwrap());
        let m = Model::new_from_model_perf(perf, states, pbs, gbs);
        self.model = m;
        Ok(())
    }
}
