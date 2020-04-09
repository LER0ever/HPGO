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

    /// Propagate a given function with a determined variable split and constraint
    /// map. Returns a new constraint map if succeeded, and None if conflict. Errors
    /// are propagated gracefully.
    pub fn propagate_bfs(
        &self,
        func_id: usize,
        p_name: &str, // NOTE: only pass parameter here, preferably non-repetitive
        split: i8,
        constraints: &HashMap<&str, HashSet<i8>>,
    ) -> Result<Option<(HashMap<&str, HashSet<i8>>)>, Box<dyn Error>> {
        // Option<(determined set, undetermined set)>

        // make a copy of constraint map, pending return.
        let m = constraints.clone();
        // NOTE: check compliance with constraint
        if m.contains_key(p_name) && m[p_name].contains(&split) {
            return Ok(None); // conflict with constraint
        }
        assert_eq!(!m.contains_key(p_name) || m[p_name].contains(&split), true);

        let f = &self.ast.functions[func_id];
        let mut v_inst_color: HashMap<usize, HashSet<usize>> = HashMap::new();
        let v_var_split: HashMap<&str, HashSet<i8>> = HashMap::new();
        assert_eq!(self.ast.var_pos[p_name].0, func_id);

        // NOTE: bfs start
        let mut q: VecDeque<(&str, i8)> = VecDeque::new();
        q.push_back((p_name, split));
        while !q.is_empty() {
            let (v, s) = q.pop_front().unwrap();
            let v_pos = &self.ast.var_pos[v].1;
            // iterate over all positions of the variable
            for vp in v_pos {
                let v_derive: Vec<(HashMap<String, i8>, usize)> =
                    self.derive(func_id, *vp, v, s)?;
                for (d, i) in v_derive {
                    // NOTE: check if we've visited the same inst at the same color
                    if v_inst_color.contains_key(vp) && v_inst_color[vp].contains(&i) {
                        continue;
                    } else if v_inst_color.contains_key(vp) {
                        v_inst_color.get_mut(vp).unwrap().insert(i);
                    } else {
                        v_inst_color.insert(*vp, [i].iter().cloned().collect());
                    }
                    // TODO: check the above logic



                }
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
    ) -> Result<Vec<(HashMap<String, i8>, usize)>, Box<dyn Error>> {
        let inst_derive = Derivation::derive(&self.derive, func_id, inst_id)?;
        let mut result: Vec<(HashMap<String, i8>, usize)> = vec![];
        inst_derive.iter().enumerate().for_each(|(i, d)| {
            if d.contains_key(var_name) && d[var_name] == split {
                // NOTE: we depend on the order of solution being constant here
                result.push((d.clone(), i));
            }
        });
        Ok(result)
    }
}
