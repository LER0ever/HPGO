use super::HLOModelImporter;
use ir::hlo_ast;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use log::{debug, error, info, trace, warn};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

const VERBOSE: bool = true;

pub struct HLOStructuredJsonImporter {}

impl HLOModelImporter for HLOStructuredJsonImporter {
    fn new() -> HLOStructuredJsonImporter {
        HLOStructuredJsonImporter {}
    }

    fn ImportFrom(&self, filename: &str) -> Result<hlo_ast::HLORoot, Box<Error>> {
        debug!("[input]\tImporting Participle Json from Go...");
        let file = File::open(Path::new(filename))?;
        let reader = BufReader::new(file);
        let ast_root = serde_json::from_reader(reader)?;
        Ok(ast_root)
    }
}
