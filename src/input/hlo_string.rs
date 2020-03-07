use super::HLOModelImporter;
use layerwise::model::model_perf;
use pyo3::prelude::*;
use pyo3::types::PyModule;

const VERBOSE: bool = true;

pub struct HLOStructuredJsonImporter {}

impl HLOModelImporter for HLOStructuredJsonImporter {
    fn new() -> HLOStructuredJsonImporter {
        HLOStructuredJsonImporter {}
    }

    fn ImportFrom(&self, filename: &str) -> () {
        unimplemented!()
    }
}
