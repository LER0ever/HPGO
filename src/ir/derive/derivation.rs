use crate::ir::error::DeriveError::*;
use std::collections::HashMap;
use std::error::Error;

use rayon::prelude::*;

use crate::ir::error::DeriveError;
use crate::ir::hlo_ast::*;
use itertools::Itertools;
use std::borrow::Borrow;
use std::env::var;

pub type Split<'a> = (&'a str, i8);

pub struct Derivation<'a> {
    pub derive_cache: HashMap<&'a Instruction, HashMap<&'a str, i8>>,
}

impl<'a> Derivation<'a> {
    pub fn new() -> Derivation<'a> {
        Derivation {
            derive_cache: HashMap::new(),
        }
    }

    pub fn cache_all_derive(&self, ast_root: &'a HLORoot) -> Result<(), Box<dyn Error>> {
        let ok = ast_root
            .functions
            .par_iter()
            .map(|f| {
                f.body
                    .par_iter()
                    .map(|i| self.cache_derive(i).is_ok())
                    .all(|x| x == true)
            })
            .all(|x| x == true);
        match ok {
            true => Ok(()),
            false => Err("Caching derivation has at least one failure...".into()),
        }
    }

    pub fn cache_derive(&self, inst: &'a Instruction) -> Result<(), Box<dyn Error>> {
        match inst.function.name {
            _ => Ok(()),
        }
    }
}
