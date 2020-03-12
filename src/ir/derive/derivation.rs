use crate::ir::error::DeriveError::*;
use std::collections::HashMap;
use std::error::Error;

use rayon::prelude::*;

use crate::ir::error::DeriveError;
use crate::ir::hlo_ast::*;
use itertools::Itertools;
use log::debug;
use std::borrow::Borrow;
use std::env::var;

pub type Split<'a> = (&'a str, i8);

pub struct Derivation<'a> {
    pub lazy_cache: bool,
    pub derive_cache: HashMap<&'a Instruction, Vec<HashMap<&'a str, i8>>>,
    pub ast: Option<&'a HLORoot>,
}

impl<'a> Derivation<'a> {
    pub fn new() -> Derivation<'a> {
        Derivation {
            derive_cache: HashMap::new(),
            lazy_cache: false,
            ast: None,
        }
    }

    pub fn new_with_ast(ast: &'a HLORoot) -> Derivation<'a> {
        let mut d = Derivation {
            derive_cache: HashMap::new(),
            lazy_cache: false,
            ast: Some(ast),
        };
        d.cache_all_derive(ast).unwrap();
        d
    }

    pub fn cache_all_derive(&mut self, ast_root: &'a HLORoot) -> Result<(), Box<dyn Error>> {
        self.derive_cache
            .par_extend(ast_root.functions.par_iter().flat_map(|f| {
                f.body
                    .par_iter()
                    .map(|i| (i, Self::d(i).unwrap_or(vec![]))) // TODO: use ? after finishing all d_s
                    .collect::<Vec<(&'a Instruction, Vec<HashMap<&'a str, i8>>)>>()
            }));
        Ok(())
    }

    pub fn derive(
        &mut self,
        inst: &'a Instruction,
    ) -> Result<Vec<HashMap<&'a str, i8>>, Box<dyn Error>> {
        if self.derive_cache.contains_key(inst) {
            Ok(self.derive_cache[inst].clone())
        } else {
            self.cache_all_derive(self.ast.ok_or(ASTNotPresent())?)?;
            assert_eq!(
                self.derive_cache.contains_key(inst),
                true,
                "Instruction cache miss even after a full AOT generation"
            );
            self.derive(inst)
        }
    }

    pub fn derive_infer(
        &self,
        inst: &'a Instruction,
    ) -> Result<&Vec<HashMap<&'a str, i8>>, Box<dyn Error>> {
        if self.derive_cache.contains_key(inst) {
            Ok(&self.derive_cache[inst])
        } else {
            Err(Box::new(InstNotInCache(format!("{:?}", inst))))
        }
    }
}
