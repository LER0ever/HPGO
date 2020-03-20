use crate::ir::error::PropagationError::*;
use crate::ir::hlo_ast::{HLOFunction, Instruction, Param};
use crate::ir::propagate::vargraph::*;
use log::debug;
use petgraph::prelude::*;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::error::Error;

const VERBOSE_THRESHOLD: usize = 700;

impl<'a> VarGraph3D<'a> {
    fn merge_with(a: &mut HashMap<&'a str, HashSet<i8>>, b: &HashMap<&'a str, HashSet<i8>>) {
        for (k, v) in b {
            if a.contains_key(k) {
                debug!("map: adding {:?} to ({}, {:?})", v, k, a[k]);
                let new_set: HashSet<i8> = a[k].union(&v).cloned().collect();
                *a.get_mut(k).unwrap() = new_set;
                debug!("map: now {} -> {:?}", k, a[k]);
            } else {
                debug!("map: new {} -> {:?}", k, v);
                a.insert(k, v.clone());
            }
        }
    }

    fn intersect_with(
        a: &mut HashMap<&'a str, HashSet<i8>>,
        b: &HashMap<&'a str, HashSet<i8>>,
    ) -> bool {
        for (k, v) in b {
            if a.contains_key(k) {
                debug!("map: adding {:?} to ({}, {:?})", v, k, a[k]);
                let new_set: HashSet<i8> = a[k].intersection(&v).cloned().collect();
                *a.get_mut(k).unwrap() = new_set;
                if a[k].len() == 0 {
                    debug!("set intersection results in empty");
                    return false;
                }
                debug!("map: now {} -> {:?}", k, a[k]);
            } else {
                debug!("map: new {} -> {:?}", k, v);
                a.insert(k, v.clone());
            }
        }

        true
    }

    pub fn propagate(
        &mut self,
        f: &'a HLOFunction,
    ) -> Result<Vec<HashMap<&'a str, HashSet<i8>>>, Box<dyn Error>> {
        let return_var = &f.body[f.body.len() - 1].var_name;
        let result = self.propagate_re(0, &HashMap::new(), &f.params, Some(return_var))?;
        Ok(result)
    }

    pub fn propagate_re(
        &mut self,
        index: usize,
        m_constraits: &HashMap<&'a str, HashSet<i8>>,
        params: &'a Vec<Param>,
        return_var: Option<&'a str>,
    ) -> Result<Vec<HashMap<&'a str, HashSet<i8>>>, Box<dyn Error>> {
        if params.len() > 300 {
            println!("re @ index {}, m.len() = {}", index, m_constraits.len(),);
        }
        let mut ret: Vec<HashMap<&'a str, HashSet<i8>>> = vec![];

        // construct the solution space for current index
        let mut dim_list: Vec<i8>;
        let param_name = params[index].name.as_str();
        if m_constraits.contains_key(param_name) {
            debug!(
                "range contraints: {} -> {}",
                params[index]
                    .param_type
                    .dimensions
                    .as_ref()
                    .unwrap_or(&vec![])
                    .len()
                    + 1,
                m_constraits[param_name].len()
            );
            dim_list = m_constraits[param_name].iter().cloned().collect();
        } else {
            dim_list = (0..params[index]
                .param_type
                .dimensions
                .as_ref()
                .unwrap_or(&vec![])
                .len() as i8)
                .collect();
            if !dim_list.contains(&-1i8) {
                dim_list.insert(0, -1i8);
            }
        }

        if index + 1 == params.len() {
            for d in dim_list.iter() {
                let node = self.get_node_id(param_name, *d);
                if node.is_none() {
                    println!("failed to get node_id for ({},{})", param_name, d);
                    continue;
                }
                let node_id = node.unwrap();
                self.visited = RefCell::new(vec![]);
                let result = self.propagate_dfs(
                    node_id,
                    HashMap::new(),
                    HashMap::new(),
                    HashMap::new(),
                    m_constraits,
                    vec![],
                    params.len() > VERBOSE_THRESHOLD, // true if
                )?;
                if let Some(m) = result {
                    ret.push(m);
                }
            }
            return Ok(ret);
        }

        for d in dim_list.iter() {
            let node = self.get_node_id(param_name, *d);
            if node.is_none() {
                println!("failed to get node_id for ({},{})", param_name, d);
                continue;
            }
            let node_id = node.unwrap();
            self.visited = RefCell::new(vec![]);
            let result = self.propagate_dfs(
                node_id,
                HashMap::new(),
                HashMap::new(),
                HashMap::new(),
                m_constraits,
                vec![],
                params.len() > VERBOSE_THRESHOLD, // true if
            )?;
            if result.is_none() {
                continue;
            }
            let m = result.unwrap();
            let sub_res = self.propagate_re(index + 1, &m, params, return_var)?;
            for ssr in sub_res {
                let mut m_copied = m.clone();
                let suc = Self::intersect_with(&mut m_copied, &ssr);
                if suc {
                    ret.push(m_copied);
                }
            }
        }

        Ok(ret)
    }

    /// Perform regulated DFS given f the starting node, and m the constraint map
    pub fn propagate_dfs(
        &self,
        f: NodeIndex,
        mut m: HashMap<&'a str, HashSet<i8>>,
        mut v_node: HashMap<&'a str, i8>,
        v_inst: HashMap<i32, i32>,
        m_constraits: &HashMap<&'a str, HashSet<i8>>,
        mut debug_chain: std::vec::Vec<(&'a str, i8)>,
        verbose: bool,
    ) -> Result<Option<HashMap<&'a str, HashSet<i8>>>, Box<dyn Error>> {
        let w = self.graph.node_weight(f).unwrap();
        if verbose {
            println!(
                "dfs({}, {}), m.len() = {}, v.len() = ({}, {})\nchain = {:?}",
                w.0,
                w.1,
                m.len(),
                v_node.len(),
                v_inst.len(),
                debug_chain,
            );
        }

        // NOTE: add the current node to v_node
        if v_node.contains_key(w.0) {
            return Err(Box::new(AlreadyVisitedIncompatible(format!(
                "Visiting a node that's already visited... ({}, {}) | ({}, {})",
                w.0, w.1, w.0, v_node[w.0]
            ))));
        } else {
            v_node.insert(w.0, w.1);
        }

        // NOTE: add the current split to m
        if m.contains_key(w.0) {
            m.get_mut(w.0).unwrap().insert(w.1);
            // m[w.0].insert(w.1);
        }
        m.insert(w.0, [w.1].iter().cloned().collect());

        if self.visited.borrow().contains(&m) {
            debug!("visited! visit len() = {}", self.visited.borrow().len());
            return Ok(None);
        }
        self.visited.borrow_mut().push(m.clone());

        // DEBUG: add node to debug_chain
        debug_chain.push((w.0, w.1));

        // NOTE: process out edges
        let next_edges = self.graph.edges(f);
        let mut suc_once = -1i8;
        // let mut sub_results: HashMap<&'a str, HashSet<i8>> = HashMap::new();
        for e in next_edges {
            let ew = e.weight();
            // NOTE: check if we've walked this instruction, but not this color
            // if so, we skip this edge due to color mismatch
            {
                let mut valid_edge = true;
                for (i, c) in ew {
                    if v_inst.contains_key(i) && !v_inst[i] == *c {
                        valid_edge = false;
                    }
                }
                if !valid_edge {
                    debug!("dfs({}, {}), abandoning edge {:?}", w.0, w.1, e.id(),);
                    continue;
                }
            }

            let edge_endpoints = self.graph.edge_endpoints(e.id()).unwrap();
            let next_node = if edge_endpoints.0 == f {
                edge_endpoints.1
            } else {
                edge_endpoints.0
            };
            // NOTE: check if we've visited other dimensions of the same node
            let nw = self.graph.node_weight(next_node).unwrap();
            if v_node.contains_key(nw.0) {
                continue;
            }

            // NOTE: check if node is within constraint
            if m_constraits.contains_key(nw.0) && !m_constraits[nw.0].contains(&nw.1) {
                continue;
            }

            // NOTE: add current edge to v_inst,
            // because we won't be able to see it in subroutines
            let mut v_inst_copied = v_inst.clone();
            for (i, c) in ew {
                if v_inst.contains_key(i) {
                    assert_eq!(v_inst[i] == *c, true);
                } else {
                    v_inst_copied.insert(*i, *c);
                }
            }

            let new_map = self.propagate_dfs(
                next_node,
                m.clone(),
                v_node.clone(),
                v_inst_copied,
                m_constraits,
                debug_chain.clone(),
                verbose,
            )?;

            // error handling, suc flag switching
            if new_map.is_none() {
                if suc_once == -1 {
                    suc_once = 0;
                }
                continue;
            }
            suc_once = 1;
            Self::merge_with(&mut m, &new_map.unwrap());
        }
        // Self::merge_with(&mut m, sub_results);
        // println!("printing m before return");
        // for (k, v) in m.iter() {
        // 	println!("{} -> {:?}", k, v);
        // }
        if suc_once == 0 {
            Ok(None)
        } else {
            Ok(Some(m))
        }
    }
}
