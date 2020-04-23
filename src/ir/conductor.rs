use crate::input::{hlo_string::HLOStructuredJsonImporter, HLOModelImporter};
use crate::ir::derive::Derivation;
use crate::ir::hlo_ast::{HLORoot, Param};
use crate::ir::propagate::ast_propagate;
use pyo3::exceptions;
use pyo3::prelude::*;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::time::{Duration, Instant};

#[pyclass]
#[derive(Debug, Clone)]
pub struct Conductor {
    #[pyo3(get)]
    pub ast: HLORoot,
    #[pyo3(get)]
    pub propagate: ast_propagate::Context,
}

#[pymethods]
impl Conductor {
    /// Construct a new Conductor for working with HLO/MLIR model planning
    #[new]
    pub fn new() -> Self {
        Conductor {
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

    pub fn get_best_split(&self, func_id: usize, params: Vec<String>) -> PyResult<()> {
        // make sure we've run positional cache, as well as fusion cache
        assert_eq!(self.ast.var_pos.len() != 0, true);
        assert_eq!(self.propagate.derive.len() != 0, true);

        let f = &self.ast.functions[func_id];
        let mut target_params: Vec<Param> = vec![];
        f.params.iter().for_each(|p| {
            if params.contains(&p.name) {
                target_params.push(p.clone());
            }
        });

        println!(
            "[propagate]\t got {} target params out of {} all",
            target_params.len(),
            params.len()
        );

        let now = Instant::now();
        self.propagate
            .get_best_split(func_id, &target_params)
            .unwrap();
        println!(
            "[propagate]\t Propagate REMT on AST Root... {}s",
            now.elapsed().as_secs()
        );

        Ok(())
    }
}
