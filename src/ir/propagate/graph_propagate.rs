use crate::ir::hlo_ast::{HLOFunction, Param};
use crate::ir::propagate::vargraph::*;
use log::debug;
use petgraph::prelude::*;
use rayon::prelude::*;
use std::collections::{BTreeMap, BTreeSet, HashSet, VecDeque};
use std::error::Error;

// const VERBOSE_THRESHOLD: usize = 1000;
// const DFS_RETURN_THRESHOLD: usize = 9000;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct State<'a> {
    visited_node: BTreeMap<&'a str, i8>,
    last_node: NodeIndex,
    visited_inst: BTreeMap<i32, i32>,
}

impl<'a> State<'a> {
    pub fn from_node(name: &'a str, dim: i8, index: NodeIndex) -> Self {
        let mut vn: BTreeMap<&'a str, i8> = BTreeMap::new();
        vn.insert(name, dim);
        State {
            visited_node: vn,
            last_node: index,
            visited_inst: BTreeMap::new(),
        }
    }

    pub fn add_node(&mut self, name: &'a str, dim: i8, index: NodeIndex) {
        self.visited_node.insert(name, dim);
        self.last_node = index;
    }

    pub fn add_edge(&mut self, inst_id: i32, edge_color: i32) {
        if self.visited_inst.contains_key(&inst_id) {
            assert_eq!(self.visited_inst[&inst_id], edge_color);
        } else {
            self.visited_inst.insert(inst_id, edge_color);
        }
    }
}

impl<'a> VarGraph3D<'a> {
    fn intersect_with(
        a: &mut BTreeMap<&'a str, BTreeSet<i8>>,
        b: &BTreeMap<&'a str, BTreeSet<i8>>,
    ) -> bool {
        for (k, v) in b {
            if a.contains_key(k) {
                debug!("map: adding {:?} to ({}, {:?})", v, k, a[k]);
                let new_set: BTreeSet<i8> = a[k].intersection(&v).cloned().collect();
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
    ) -> Result<Vec<BTreeMap<&'a str, BTreeSet<i8>>>, Box<dyn Error>> {
        let return_var = &f.body[f.body.len() - 1].var_name;
        let result = self.propagate_remt(0, &BTreeMap::new(), &f.params, Some(return_var))?;
        Ok(result)
    }

    pub fn propagate_remt(
        &self,
        index: usize,
        m_constraits: &BTreeMap<&'a str, BTreeSet<i8>>,
        params: &'a Vec<Param>,
        return_var: Option<&'a str>,
    ) -> Result<Vec<BTreeMap<&'a str, BTreeSet<i8>>>, Box<dyn Error>> {
        if params.len() > 300 {
            println!(
                "remt {} @ index {}, m.len() = {}",
                params[index].name.as_str(),
                index,
                m_constraits.len(),
            );
        }
        let mut ret: Vec<BTreeMap<&'a str, BTreeSet<i8>>> = vec![];

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
                dim_list.push(-1i8);
            }
        }

        if index + 1 == params.len() {
            ret.par_extend(
                dim_list
                    .par_iter()
                    .map(|d| {
                        let node = self.get_node_id(param_name, *d);
                        if node.is_none() {
                            println!("failed to get node_id for ({},{})", param_name, d);
                            return None;
                        }
                        let node_id = node.unwrap();
                        let result = self
                            .propagate_bfs(node_id, m_constraits, params.len() > 300)
                            .unwrap();
                        if params.len() > 300 {
                            println!("result += {:?}", result);
                        }
                        return Some(result);
                        // ret.push(result);
                    })
                    .filter(|x| x.is_some())
                    .map(|x| x.unwrap()),
            );

            return Ok(ret);
        }

        ret.par_extend(
            dim_list
                .par_iter()
                .flat_map(|d| {
                    let node = self.get_node_id(param_name, *d);
                    if node.is_none() {
                        println!("[remt]\t failed to get node_id for ({},{})", param_name, d);
                        return vec![];
                    }
                    let node_id = node.unwrap();
                    let m = self
                        .propagate_bfs(node_id, m_constraits, params.len() > 300)
                        .unwrap();

                    if params.len() > 300 {
                        let mc = m.clone();
                        for p in params.iter() {
                            if mc.contains_key(p.name.as_str()) {
                                print!("\"{}\": {:?}, ", p.name.as_str(), mc[p.name.as_str()]);
                            }
                        }
                        println!("");
                    }
                    let sub_res = self
                        .propagate_remt(index + 1, &m, params, return_var)
                        .unwrap();
                    let mut sub_ret: Vec<BTreeMap<&'a str, BTreeSet<i8>>> = vec![];
                    for ssr in sub_res {
                        let mut m_copied = m.clone();
                        let suc = Self::intersect_with(&mut m_copied, &ssr);
                        if suc {
                            sub_ret.push(m_copied);
                        }
                    }
                    return sub_ret;
                })
                .collect::<Vec<BTreeMap<&'a str, BTreeSet<i8>>>>(),
        );

        Ok(ret)
    }

    // pub fn propagate_re(
    //     &mut self,
    //     index: usize,
    //     m_constraits: &BTreeMap<&'a str, BTreeSet<i8>>,
    //     params: &'a Vec<Param>,
    //     return_var: Option<&'a str>,
    // ) -> Result<Vec<BTreeMap<&'a str, BTreeSet<i8>>>, Box<dyn Error>> {
    //     if params.len() > 300 {
    //         println!(
    //             "re {} @ index {}, m.len() = {}",
    //             params[index].name.as_str(),
    //             index,
    //             m_constraits.len(),
    //         );
    //     }
    //     let mut ret: Vec<BTreeMap<&'a str, BTreeSet<i8>>> = vec![];
    //
    //     let bfs_switch = true;
    //
    //     // construct the solution space for current index
    //     let mut dim_list: Vec<i8>;
    //     let param_name = params[index].name.as_str();
    //     if m_constraits.contains_key(param_name) {
    //         debug!(
    //             "range contraints: {} -> {}",
    //             params[index]
    //                 .param_type
    //                 .dimensions
    //                 .as_ref()
    //                 .unwrap_or(&vec![])
    //                 .len()
    //                 + 1,
    //             m_constraits[param_name].len()
    //         );
    //         dim_list = m_constraits[param_name].iter().cloned().collect();
    //     } else {
    //         dim_list = (0..params[index]
    //             .param_type
    //             .dimensions
    //             .as_ref()
    //             .unwrap_or(&vec![])
    //             .len() as i8)
    //             .collect();
    //         if !dim_list.contains(&-1i8) {
    //             dim_list.push(-1i8);
    //         }
    //     }
    //
    //     if index + 1 == params.len() {
    //         for d in dim_list.iter() {
    //             let node = self.get_node_id(param_name, *d);
    //             if node.is_none() {
    //                 println!("failed to get node_id for ({},{})", param_name, d);
    //                 continue;
    //             }
    //             let node_id = node.unwrap();
    //             if bfs_switch {
    //                 let result = self.propagate_bfs(node_id, m_constraits, params.len() > 300)?;
    //                 if params.len() > 300 {
    //                     println!("result += {:?}", result);
    //                 }
    //                 ret.push(result);
    //             } else {
    //                 self.visited = RefCell::new(HashSet::new());
    //                 let result = self.propagate_dfs(
    //                     node_id,
    //                     BTreeMap::new(),
    //                     HashMap::new(),
    //                     HashMap::new(),
    //                     m_constraits,
    //                     vec![],
    //                     params.len() > VERBOSE_THRESHOLD, // true if
    //                 )?;
    //                 if let Some(m) = result {
    //                     if params.len() > 300 {
    //                         println!("result += {:?}", m);
    //                     }
    //
    //                     ret.push(m);
    //                 }
    //             }
    //         }
    //         return Ok(ret);
    //     }
    //
    //     for d in dim_list.iter() {
    //         let node = self.get_node_id(param_name, *d);
    //         if node.is_none() {
    //             println!("failed to get node_id for ({},{})", param_name, d);
    //             continue;
    //         }
    //         let node_id = node.unwrap();
    //         let mut m: BTreeMap<&'a str, BTreeSet<i8>> = BTreeMap::new();
    //         if bfs_switch {
    //             m = self.propagate_bfs(node_id, m_constraits, params.len() > 300)?;
    //         } else {
    //             self.visited = RefCell::new(HashSet::new());
    //             if params.len() > 300 {
    //                 println!("dfs {} , {}", param_name, *d);
    //             }
    //
    //             let result = self.propagate_dfs(
    //                 node_id,
    //                 BTreeMap::new(),
    //                 HashMap::new(),
    //                 HashMap::new(),
    //                 m_constraits,
    //                 vec![],
    //                 params.len() > VERBOSE_THRESHOLD, // true if
    //             )?;
    //             if result.is_none() {
    //                 continue;
    //             }
    //             m = result.unwrap();
    //         }
    //         if params.len() > 300 {
    //             let mc = m.clone();
    //             for p in params.iter() {
    //                 if mc.contains_key(p.name.as_str()) {
    //                     print!("\"{}\": {:?}, ", p.name.as_str(), mc[p.name.as_str()]);
    //                 }
    //             }
    //             println!("");
    //         }
    //         let sub_res = self.propagate_re(index + 1, &m, params, return_var)?;
    //         for ssr in sub_res {
    //             let mut m_copied = m.clone();
    //             let suc = Self::intersect_with(&mut m_copied, &ssr);
    //             if suc {
    //                 ret.push(m_copied);
    //             }
    //         }
    //     }
    //
    //     Ok(ret)
    // }
    //
    // /// Perform regulated DFS given f the starting node, and m the constraint map
    // pub fn propagate_dfs(
    //     &self,
    //     f: NodeIndex,
    //     mut m: BTreeMap<&'a str, BTreeSet<i8>>,
    //     mut v_node: HashMap<&'a str, i8>,
    //     v_inst: HashMap<i32, i32>,
    //     m_constraits: &BTreeMap<&'a str, BTreeSet<i8>>,
    //     mut debug_chain: std::vec::Vec<(&'a str, i8)>,
    //     verbose: bool,
    // ) -> Result<Option<BTreeMap<&'a str, BTreeSet<i8>>>, Box<dyn Error>> {
    //     let w = self.graph.node_weight(f).unwrap();
    //     if (verbose && debug_chain.len() < 500) {
    //         let indent = (0..debug_chain.len() / 4).map(|_| " ").collect::<String>();
    //         println!(
    //             "{}{}dfs({}, {}), m.len() = {}, v.len() = ({}, {}), visited.len() = {}",
    //             debug_chain.len(),
    //             indent,
    //             w.0,
    //             w.1,
    //             m.len(),
    //             v_node.len(),
    //             v_inst.len(),
    //             self.visited.borrow().len(),
    //         );
    //     }
    //     if self.visited.borrow().len() >= DFS_RETURN_THRESHOLD {
    //         return Ok(Some(m));
    //     }
    //
    //     // NOTE: add the current node to v_node
    //     if v_node.contains_key(w.0) {
    //         return Err(Box::new(AlreadyVisitedIncompatible(format!(
    //             "Visiting a node that's already visited... ({}, {}) | ({}, {})",
    //             w.0, w.1, w.0, v_node[w.0]
    //         ))));
    //     } else {
    //         v_node.insert(w.0, w.1);
    //     }
    //
    //     // NOTE: add the current split to m
    //     if m.contains_key(w.0) {
    //         m.get_mut(w.0).unwrap().insert(w.1);
    //         // m[w.0].insert(w.1);
    //     }
    //     m.insert(w.0, [w.1].iter().cloned().collect());
    //
    //     if self.visited.borrow().contains(&m) {
    //         debug!("visited! visit len() = {}", self.visited.borrow().len());
    //         return Ok(None);
    //     }
    //     self.visited.borrow_mut().insert(m.clone());
    //
    //     // DEBUG: add node to debug_chain
    //     debug_chain.push((w.0, w.1));
    //
    //     // NOTE: process out edges
    //     let next_edges = self.graph.edges(f);
    //     let mut suc_once = -1i8;
    //     // let mut sub_results: HashMap<&'a str, HashSet<i8>> = HashMap::new();
    //     for e in next_edges {
    //         let ew = e.weight();
    //         // NOTE: check if we've walked this instruction, but not this color
    //         // if so, we skip this edge due to color mismatch
    //         {
    //             let mut valid_edge = true;
    //             for (i, c) in ew {
    //                 if v_inst.contains_key(i) && v_inst[i] != *c {
    //                     valid_edge = false;
    //                 }
    //             }
    //             if !valid_edge {
    //                 debug!("dfs({}, {}), abandoning edge {:?}", w.0, w.1, e.id(),);
    //                 continue;
    //             }
    //         }
    //
    //         let edge_endpoints = self.graph.edge_endpoints(e.id()).unwrap();
    //         let next_node = if edge_endpoints.0 == f {
    //             edge_endpoints.1
    //         } else {
    //             edge_endpoints.0
    //         };
    //         // NOTE: check if we've visited other dimensions of the same node
    //         let nw = self.graph.node_weight(next_node).unwrap();
    //         if v_node.contains_key(nw.0) {
    //             continue;
    //         }
    //
    //         // NOTE: check if node is within constraint
    //         if m_constraits.contains_key(nw.0) && !m_constraits[nw.0].contains(&nw.1) {
    //             continue;
    //         }
    //
    //         // NOTE: add current edge to v_inst,
    //         // because we won't be able to see it in subroutines
    //         let mut v_inst_copied = v_inst.clone();
    //         for (i, c) in ew {
    //             if v_inst.contains_key(i) {
    //                 // println!("v_inst check {} vs {}", v_inst[i], c);
    //                 assert_eq!(v_inst[i] == *c, true);
    //             } else {
    //                 v_inst_copied.insert(*i, *c);
    //             }
    //         }
    //
    //         let new_map = self.propagate_dfs(
    //             next_node,
    //             m.clone(),
    //             v_node.clone(),
    //             v_inst_copied,
    //             m_constraits,
    //             debug_chain.clone(),
    //             verbose,
    //         )?;
    //
    //         // error handling, suc flag switching
    //         if new_map.is_none() {
    //             if suc_once == -1 {
    //                 suc_once = 0;
    //             }
    //             continue;
    //         }
    //         suc_once = 1;
    //         Self::merge_with(&mut m, &new_map.unwrap());
    //     }
    //     // Self::merge_with(&mut m, sub_results);
    //     // println!("printing m before return");
    //     // for (k, v) in m.iter() {
    //     // 	println!("{} -> {:?}", k, v);
    //     // }
    //     if suc_once == 0 {
    //         Ok(None)
    //     } else {
    //         Ok(Some(m))
    //     }
    // }

    #[allow(dead_code)]
    pub fn propagate_bsids(
        &self,
        f: NodeIndex,
        m_constraints: &BTreeMap<&'a str, BTreeSet<i8>>,
        verbose: bool,
    ) {
        // IDS globals
        #[allow(unused_mut)]
        let mut m: BTreeMap<&'a str, BTreeSet<i8>> = BTreeMap::new();
        let mut q: VecDeque<State<'a>> = VecDeque::new();
        let mut v: HashSet<NodeType<'a>> = HashSet::new();

        // first node
        let w = self.graph.node_weight(f).unwrap();
        let cur_state = State::from_node(w.0, w.1, f);
        v.insert(*w);
        q.push_back(cur_state);

        // search
        while !q.is_empty() {
            let s = q.pop_front().unwrap();
            let cn = self.graph.node_weight(s.last_node).unwrap();
            v.insert((cn.0, cn.1));
        }

        unimplemented!()
    }

    pub fn propagate_bfs(
        &self,
        f: NodeIndex,
        m_constraits: &BTreeMap<&'a str, BTreeSet<i8>>,
        verbose: bool,
    ) -> Result<BTreeMap<&'a str, BTreeSet<i8>>, Box<dyn Error>> {
        // State: 1. All Visited Nodes, 2. Last Visited Node, 3. Visited Inst and Color
        //type State<'a> = (BTreeMap<&'a str, i8>, &'a str, BTreeMap<i32, i32>);

        // bfs globals
        let mut m: BTreeMap<&'a str, BTreeSet<i8>> = BTreeMap::new();
        let mut q: VecDeque<State<'a>> = VecDeque::new();
        let mut v: HashSet<State<'a>> = HashSet::new();
        let mut v_node: HashSet<(&'a str, i8)> = HashSet::new();
        // NOTE: could change one of the above to be references, saves 1/2 space

        // first node
        let w = self.graph.node_weight(f).unwrap();
        let cur_state = State::from_node(w.0, w.1, f);
        v.insert(cur_state.clone());
        q.push_back(cur_state);
        let mut max_depth = 1;
        let mut iter = 0;
        while !q.is_empty() {
            let s = q.pop_front().unwrap();
            let cn = self.graph.node_weight(s.last_node).unwrap();
            v_node.insert((cn.0, cn.1));
            iter += 1;
            if verbose {
                let cur_depth = v_node.len();
                if cur_depth >= max_depth {
                    max_depth = cur_depth;
                    println!(
                        "bfs({}, {}) L{}, v_node: {}, q: {}",
                        w.0,
                        w.1,
                        cur_depth,
                        v_node.len(),
                        q.len(),
                    );
                } else {
                    // if iter % 100 == 0 {
                    //     print!(".");
                    //     io::stdout().flush();
                    // }
                }
            }

            // add cn to m
            if m.contains_key(cn.0) {
                m.get_mut(cn.0).unwrap().insert(cn.1);
            } else {
                m.insert(cn.0, [cn.1].iter().cloned().collect());
            }

            let next_edges = self.graph.edges(s.last_node);
            for e in next_edges {
                let ew = e.weight();
                // NOTE: check if we've walked this inst, but with different color
                let mut valid_edge = true;
                for (i, c) in ew {
                    if s.visited_inst.contains_key(i) && s.visited_inst[i] != *c {
                        valid_edge = false;
                    }
                    if self.color_connect.contains_key(c) {
                        let same_color_nodes = self.color_connect[c].clone();
                        for x in same_color_nodes {
                            let xw = self.graph.node_weight(x).unwrap();
                            if s.visited_node.contains_key(xw.0) && s.visited_node[xw.0] != xw.1 {
                                valid_edge = false;
                            }
                            if m_constraits.contains_key(xw.0)
                                && !m_constraits[xw.0].contains(&xw.1)
                            {
                                valid_edge = false;
                            }
                        }
                    }
                    if self.color_cover.contains_key(c) {
                        let same_color_edges = self.color_cover[c].clone();
                        for x in same_color_edges {
                            let xw = self.graph.edge_weight(x).unwrap();
                            for (xi, xc) in xw {
                                if s.visited_inst.contains_key(xi) && s.visited_inst[xi] != *xc {
                                    valid_edge = false;
                                }
                            }
                        }
                    }
                }
                if !valid_edge {
                    debug!("bfs({}, {}), abandoning edge {:?}", cn.0, cn.1, e.id());
                    continue;
                }

                // NOTE: get next node
                let edge_endpoints = self.graph.edge_endpoints(e.id()).unwrap();
                let next_node = if edge_endpoints.0 == s.last_node {
                    edge_endpoints.1
                } else {
                    edge_endpoints.0
                };
                let nw = self.graph.node_weight(next_node).unwrap();

                if v_node.contains(nw) {
                    continue;
                }

                // NOTE: check if we've visited other dimensions of this node
                if s.visited_node.contains_key(nw.0) {
                    continue;
                }

                // NOTE: check if node is within constraints
                if m_constraits.contains_key(nw.0) && !m_constraits[nw.0].contains(&nw.1) {
                    continue;
                }

                // NOTE: add current edge to v_inst,

                for (i, c) in ew {
                    let mut next_state = s.clone();
                    next_state.add_edge(*i, *c);
                    if self.color_connect.contains_key(c) {
                        let same_color_nodes = self.color_connect[c].clone();
                        for x in same_color_nodes {
                            let xw = self.graph.node_weight(x).unwrap();
                            next_state.visited_node.insert(xw.0, xw.1);
                        }
                    }
                    if self.color_cover.contains_key(c) {
                        let same_color_edges = self.color_cover[c].clone();
                        for x in same_color_edges {
                            let xw = self.graph.edge_weight(x).unwrap();
                            for (xi, xc) in xw {
                                next_state.visited_inst.insert(*xi, *xc);
                            }
                        }
                    }
                    next_state.add_node(nw.0, nw.1, next_node);
                    if !v.contains(&next_state) {
                        v.insert(next_state.clone());
                        q.push_back(next_state);
                    } else {
                        debug!(
                            "bfs({}, {} | L{}) state visited: {:?}",
                            w.0,
                            w.1,
                            next_state.visited_node.len(),
                            next_state.last_node
                        );
                    }
                }
            } // end for e in next_edges
        }

        return Ok(m);
    }
}
