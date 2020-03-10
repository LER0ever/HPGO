use super::HLOModelImporter;
use crate::ir::hlo_ast;

use log::debug;
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

    fn ImportFrom(&self, filename: &str) -> Result<hlo_ast::HLORoot, Box<dyn Error>> {
        debug!("[input]\tImporting Participle Json from Go...");
        let file = File::open(Path::new(filename))?;
        let reader = BufReader::new(file);
        let ast_root = serde_json::from_reader(reader)?;
        Ok(ast_root)
    }
}
