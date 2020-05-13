use super::HLOModelImporter;
use crate::ir::hlo_ast;

use crate::ir::hlo_ast::HLORoot;
use log::debug;
use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Instant;

#[allow(dead_code)]
const VERBOSE: bool = true;

pub struct HLOStructuredJsonImporter {}

impl HLOModelImporter for HLOStructuredJsonImporter {
    fn new() -> HLOStructuredJsonImporter {
        HLOStructuredJsonImporter {}
    }

    fn ImportFrom(&self, filename: &str) -> Result<hlo_ast::HLORoot, Box<dyn Error>> {
        debug!("[input]\tImporting Participle Json from Go...");
        let now = Instant::now();
        let file = File::open(Path::new(filename))?;
        let reader = BufReader::new(file);
        let mut ast_root: HLORoot = serde_json::from_reader(reader)?;
        // augment function param name with %
        ast_root.functions.par_iter_mut().for_each(|f| {
            f.params.iter_mut().for_each(|p| {
                p.augment_name();
            });
        });
        println!(
            "[input]\t Reading from Go-generated Participle Json... {}ms",
            now.elapsed().as_millis()
        );
        Ok(ast_root)
    }
}
