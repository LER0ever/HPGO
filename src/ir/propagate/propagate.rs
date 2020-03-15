use crate::ir::error::PropagationError::*;
use crate::ir::hlo_ast::{HLOFunction, Instruction};
use crate::ir::propagate::vargraph::*;
use petgraph::prelude::*;
use std::collections::{HashMap, HashSet};
use std::error::Error;

pub struct Propagate<'a> {
    pub vargraph: &'a VarGraph3D<'a>,
    pub ungraph: &'a UnGraph<NodeType<'a>, EdgeType<'a>>,
    // pub v_node: HashMap<&'a str, HashSet<i8>>,
    // pub v_inst: HashMap<&'a Instruction, EdgeColor<'a>>,
}

impl<'a> Propagate<'a> {
    pub fn new(g: &'a VarGraph3D<'a>) -> Propagate<'a> {
        Propagate {
            vargraph: g,
            ungraph: &g.graph,
        }
    }

    pub fn merge_with(a: &mut HashMap<&'a str, HashSet<i8>>, b: HashMap<&'a str, HashSet<i8>>) {
        for (k, v) in b.iter() {
            if a.contains_key(k) {
                a[k].union(&v);
            } else {
                a.insert(k, v.clone());
            }
        }
    }

    pub fn propagate(
        &mut self,
        _f: &'a HLOFunction,
        start_node: NodeIndex,
    ) -> Result<HashMap<&'a str, HashSet<i8>>, Box<dyn Error>> {
        let result =
            self.propagate_dfs(start_node, HashMap::new(), HashMap::new(), HashMap::new())?;
        Ok(result)
    }

    /// Perform regulated DFS given f the starting node, and m the constraint map
    pub fn propagate_dfs(
        &self,
        f: NodeIndex,
        mut m: HashMap<&'a str, HashSet<i8>>,
        mut v_node: HashMap<&'a str, i8>,
        v_inst: HashMap<&'a Instruction, EdgeColor<'a>>,
    ) -> Result<HashMap<&'a str, HashSet<i8>>, Box<dyn Error>> {
        let w = self.ungraph.node_weight(f).unwrap();
        println!(
            "dfs({}, {}), m.len() = {}, v.len() = ({}, {})",
            w.0,
            w.1,
            m.len(),
            v_node.len(),
            v_inst.len()
        );
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

        // NOTE: process out edges
        let next_edges = self.ungraph.edges(f);
        for e in next_edges {
            let ew = e.weight();
            // NOTE: check if we've walked this instruction, but not this color
            // if so, we skip this edge due to color mismatch
            {
                let mut valid_edge = true;
                for (i, c) in ew {
                    if v_inst.contains_key(i) && v_inst[i].eq(c) {
                        valid_edge = false;
                    }
                }
                if !valid_edge {
                    continue;
                }
            }
            // TODO: check if node is within constraint
            {}

            let edge_endpoints = self.ungraph.edge_endpoints(e.id()).unwrap();
            let next_node = if edge_endpoints.0 == f {
                edge_endpoints.1
            } else {
                edge_endpoints.0
            };
            // NOTE: check if we've visited other dimensions of the same node
            let nw = self.ungraph.node_weight(next_node).unwrap();
            if v_node.contains_key(nw.0) {
                continue;
            }

            // NOTE: add current edge to v_inst,
            // because we won't be able to see it in subroutines
            let mut v_inst_copied = v_inst.clone();
            for (i, c) in ew {
                if v_inst.contains_key(i) {
                    assert_eq!(v_inst[i].eq(c), true);
                } else {
                    v_inst_copied.insert(i, c);
                }
            }

            let new_map =
                self.propagate_dfs(next_node, m.clone(), v_node.clone(), v_inst_copied)?;
            Self::merge_with(&mut m, new_map);
        }
        Ok(m)
    }
}

// impl<'a> VarGraph3D<'a> {
//     /// propagates the given function, and returns the resulting posible splits,
//     /// as if it was a single instruction with a function call.
//     pub fn propagate(
//         &mut self,
//         f: &'a HLOFunction,
//     ) -> Result<HashMap<&'a str, HashSet<i8>>, Box<dyn Error>> {
//         let node_id = self.node_id(&f.params[0].name, 0i8);
//         let p = Propagate::new(&self);
//         let result = p.propagate_dfs(node_id, HashMap::new(), HashMap::new(), HashMap::new())?;
//         Ok(result.clone())
//     }
// }
