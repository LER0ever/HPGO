use crate::ir::error::DeriveError::*;
use crate::ir::hlo_ast::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::time::{Duration, Instant};

pub type Split<'a> = (&'a str, i8);

#[derive(Debug, Clone)]
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

    pub fn cache_ast(
        ast_root: &'a HLORoot,
    ) -> Result<Vec<(&'a Instruction, Vec<HashMap<&'a str, i8>>)>, Box<dyn Error>> {
        let result = ast_root
            .functions
            .par_iter()
            .flat_map(|f| {
                f.body
                    .par_iter()
                    .map(|i| (i, Self::d(i).unwrap())) // TODO: use ? after finishing all d_s
                    .collect::<Vec<(&'a Instruction, Vec<HashMap<&'a str, i8>>)>>()
            })
            .collect::<Vec<(&'a Instruction, Vec<HashMap<&'a str, i8>>)>>();
        Ok(result)
    }

    pub fn cache_all_derive(&mut self, ast_root: &'a HLORoot) -> Result<(), Box<dyn Error>> {
        let now = Instant::now();
        self.derive_cache.par_extend(Self::cache_ast(ast_root)?);
        println!(
            "[derivation]\t Cache Derivation for AST... {}ms",
            now.elapsed().as_millis()
        );
        Ok(())
    }

    pub fn cache_export(
        &self,
    ) -> Result<HashMap<Instruction, Vec<HashMap<String, i8>>>, Box<dyn Error>> {
        if self.derive_cache.len() == 0 {
            println!("[derive]\t cache_all_derive not run before trying to get the result");
            return Err(Box::new(CacheNotAvailable()));
        }
        let mut result: HashMap<Instruction, Vec<HashMap<String, i8>>> = HashMap::new();
        result.par_extend(self.derive_cache.par_iter().map(|(k, v)| {
            let inst: Instruction = k.clone().to_owned();
            let mut v_exp: Vec<HashMap<String, i8>> = vec![];
            for m in v {
                let mut m_exp: HashMap<String, i8> = HashMap::new();
                for (kk, vv) in m {
                    m_exp.insert(kk.to_string(), *vv);
                }
                v_exp.push(m_exp);
            }
            (inst, v_exp)
        }));
        Ok(result)
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
