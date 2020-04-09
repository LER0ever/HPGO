use crate::ir::error::PropagationError::*;
use crate::ir::hlo_ast::{HLORoot};
use crate::ir::derive::{Derivation, DeriveCache};
use log::debug;
use std::error::Error;
use std::time::{Duration, Instant};
use petgraph::prelude::*;
use rayon::prelude::*;

pub struct Context {
    ast: HLORoot,
    derive: DeriveCache,
}

impl Context {
    pub fn new(ast: HLORoot) -> Self {
        assert_eq!(ast.inst_pos.len() > 0, true);
        let mut d = Derivation::new_with_ast(&ast);
        let cache_result = d.cache_export().unwrap();
        Context {
            ast: ast,
            derive: cache_result,
        }
    }

    pub fn propagate(&self, func_id: usize) -> Result<bool, Box<dyn Error>> {
        let f = &self.ast.functions[func_id];
        unimplemented!()
    }
}
