use crate::ir::derive::{Derivation, DeriveCache};
use crate::ir::error::PropagationError::*;
use crate::ir::hlo_ast::HLORoot;
use log::debug;
use petgraph::prelude::*;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::ops::Deref;
use std::time::{Duration, Instant};

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
            ast,
            derive: cache_result,
        }
    }

    pub fn propagate(&self, func_id: usize) -> Result<bool, Box<dyn Error>> {
        let f = &self.ast.functions[func_id];
        unimplemented!()
    }

    pub fn propagate_bfs(
        &self,
        func_id: usize,
        p_name: &str, // NOTE: only pass
        split: i8,
        constraints: &HashMap<&str, HashSet<i8>>,
    ) -> Result<Option<(HashMap<&str, HashSet<i8>>)>, Box<dyn Error>> {
        // Option<(determined set, undetermined set)>

        // make a copy of constraint map, pending return.
        let m = constraints.clone();
        // NOTE: check compliance with constraint
        assert_eq!(!m.contains_key(p_name) || m[p_name].contains(&split), true);

        let f = &self.ast.functions[func_id];
        let v_inst_color: HashMap<usize, HashSet<usize>> = HashMap::new();
        let v_var_split: HashMap<&str, HashSet<i8>> = HashMap::new();
        assert_eq!(self.ast.var_pos[p_name].0, func_id);

        // NOTE: bfs start
        let mut q: VecDeque<(&str, i8)> = VecDeque::new();
        q.push_back((p_name, split));
        while !q.is_empty() {
            let (v, s) = q.pop_front().unwrap();
            let v_pos = &self.ast.var_pos[v].1;
            for vp in v_pos {

            }
        }

        unimplemented!()
    }

    fn derive(
        &self,
        func_id: usize,
        inst_id: usize,
        var_name: &str,
        split: i8,
    ) -> Result<Vec<HashMap<String, i8>>, Box<dyn Error>> {
        let inst_derive = Derivation::derive(&self.derive, func_id, inst_id)?;
        let mut result: Vec<HashMap<String, i8>> = vec![];
        for d in inst_derive {
            if d.contains_key(var_name) && d[var_name] == split {
                result.push(d.clone());
            }
        }
        Ok(result)
    }
}
