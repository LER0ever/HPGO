use crate::input::{hlo_string::HLOStructuredJsonImporter, HLOModelImporter};
use crate::ir::derive::Derivation;
use crate::ir::hlo_ast::HLORoot;
use crate::ir::propagate::ast_propagate;
use pyo3::exceptions;
use pyo3::prelude::*;
use std::collections::{HashMap, HashSet};
use std::error::Error;

#[pyclass]
#[derive(Debug, Clone)]
pub struct IRConductor {
    #[pyo3(get)]
    pub ast: HLORoot,
    #[pyo3(get)]
    pub propagate: ast_propagate::Context,
}

#[pymethods]
impl IRConductor {
    /// Construct a new IRConductor for working with HLO/MLIR model planning
    #[new]
    pub fn new() -> Self {
        IRConductor {
            ast: HLORoot::default(),
            propagate: ast_propagate::Context::default(),
        }
    }

    /// Import the model information from `filename`
    pub fn import_from(&mut self, filename: String) -> PyResult<()> {
        let hlo_importer: HLOStructuredJsonImporter = HLOModelImporter::new();
        self.ast = hlo_importer.ImportFrom(filename.as_str()).unwrap();
        self.ast.cache_all_positional().unwrap();
        let p = ast_propagate::Context::new(self.ast.clone());
        self.propagate = p;
        Ok(())
    }

    /// Wrapper for update_fusion_derive_cache
    pub fn update_fusion_for(&mut self, func_id: usize) -> PyResult<()> {
        self.propagate.update_fusion_derive_cache(func_id).unwrap();
        Ok(())
    }

    pub fn derive_enum(
        &self,
        func_id: usize,
        inst_id: usize,
    ) -> PyResult<Vec<HashMap<String, i8>>> {
        let inst_derive = Derivation::derive(&self.propagate.derive, func_id, inst_id).unwrap();
        Ok(inst_derive.clone())
    }

    pub fn derive_infer(
        &self,
        func_id: usize,
        inst_id: usize,
        var_name: String,
        split: i8,
    ) -> PyResult<Vec<(HashMap<String, i8>, usize)>> {
        let inst_derive = self
            .propagate
            .derive(func_id, inst_id, &var_name, split)
            .unwrap();
        Ok(inst_derive)
    }
}
