use crate::input::{hlo_string::HLOStructuredJsonImporter, HLOModelImporter};
use crate::ir::derive::Derivation;
use crate::ir::hlo_ast::HLORoot;
use pyo3::exceptions;
use pyo3::prelude::*;
use std::error::Error;

#[pyclass]
#[derive(Debug, Clone)]
pub struct IRConductor {
    #[pyo3(get)]
    pub ast: HLORoot,
}

impl IRConductor {
    /// Construct a new IRConductor for working with HLO/MLIR model planning
    pub fn new() -> Self {
        IRConductor {
            ast: HLORoot::default(),
        }
    }

    /// Import the model information from `filename`
    pub fn import_from(&mut self, filename: String) -> Result<(), Box<dyn Error>> {
        let hlo_importer: HLOStructuredJsonImporter = HLOModelImporter::new();
        self.ast = hlo_importer.ImportFrom(filename.as_str())?;
        Ok(())
    }
}
