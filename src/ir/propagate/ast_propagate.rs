use crate::ir::derive::{Derivation, DeriveCache};
use crate::ir::error::PropagationError::*;
use crate::ir::hlo_ast::{HLORoot, Param};
use log::debug;
use petgraph::prelude::*;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::ops::Deref;
use std::time::{Duration, Instant};
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug, Default)]
pub struct Context {
    ast: HLORoot,
    #[pyo3(get)]
    pub derive: DeriveCache,
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
        constraints: &HashMap<String, HashSet<i8>>,
    ) -> Result<Option<(HashMap<String, HashSet<i8>>)>, Box<dyn Error>> {
        // Option<(determined set, undetermined set)>
        let BFS_DEBUG = true;

        // make a copy of constraint map, pending return.
        let mut m = constraints.clone();
        // NOTE: check compliance with constraint
        if m.contains_key(p_name) && !m[p_name].contains(&split) {
            return Ok(None); // conflict with constraint
        }
        assert_eq!(!m.contains_key(p_name) || m[p_name].contains(&split), true);

        let f = &self.ast.functions[func_id];
        let mut v_inst_color: HashMap<usize, HashSet<usize>> = HashMap::new();
        let v_var_split: HashMap<&str, HashSet<i8>> = HashMap::new();
        assert_eq!(self.ast.var_pos[p_name].0, func_id);

        // NOTE: bfs start
        let mut q: VecDeque<(String, i8)> = VecDeque::new();
        q.push_back((p_name.to_string(), split));
        while !q.is_empty() {
            let (v, s) = q.pop_front().unwrap();
            let v_pos = &self.ast.var_pos[&v].1;
            if BFS_DEBUG {
                println!("\tbfs({}, {}) exploring {} positions", v, s, v_pos.len());
            }
            // iterate over all positions of the variable
            for vp in v_pos {
                // if BFS_DEBUG {
                //     println!("exploring inst ({}, {})", func_id, vp);
                // }
                let v_derive: Vec<(HashMap<String, i8>, usize)> =
                    self.derive(func_id, *vp, &v, s)?;
                // if BFS_DEBUG {
                //     println!("v_derive has {} entries", v_derive.len());
                // }
                let mut derive_aggregated: HashMap<String, HashSet<i8>> = HashMap::new();
                // let mut v_inst_valid = true;
                // for (d, i) in v_derive {
                //     // NOTE: check if we've visited the same inst at the same color
                //     if v_inst_color.contains_key(vp) && v_inst_color[vp].contains(&i) {
                //         v_inst_valid = false;
                //         continue;
                //     } else if v_inst_color.contains_key(vp) {
                //         v_inst_color.get_mut(vp).unwrap().insert(i);
                //     } else {
                //         v_inst_color.insert(*vp, [i].iter().cloned().collect());
                //     }
                //     // TODO: check the above logic
                // }
                for (d, i) in v_derive {
                    for (d_k, d_v) in d {
                        if derive_aggregated.contains_key(&d_k) {
                            derive_aggregated.get_mut(&d_k).unwrap().insert(d_v);
                        } else {
                            derive_aggregated.insert(d_k, [d_v].iter().cloned().collect());
                        }
                    }
                }
                // if BFS_DEBUG {
                //     println!("aggregated derive has {} entries", derive_aggregated.len());
                // }

                for (d_k, d_v) in derive_aggregated {
                    if d_k == v {
                        continue;
                    }
                    // if BFS_DEBUG {
                    //     println!("d_k: {}, d_v: {:?}", d_k, d_v );
                    // }

                    if d_v.len() == 1 {
                        let d_value = d_v.iter().next().unwrap();
                        let d_kstr = d_k.as_str();
                        if m.contains_key(d_kstr) && !m[d_kstr].contains(d_value) {
                            // split into conflicting dimension
                            return Ok(None);
                        } else if m.contains_key(d_kstr)
                            && m[d_kstr].contains(d_value)
                            && m[d_kstr].len() == 1
                        {
                            // split into the same dimension, usually means we've explored this var+split combination
                            // TODO: check if we need to add this back into queue
                        } else if m.contains_key(d_kstr)
                            && m[d_kstr].contains(d_value)
                            && m[d_kstr].len() > 1
                        {
                            // split into compliant dimension, need to shrink the set to single element
                            *m.get_mut(d_kstr).unwrap() = d_v.clone();
                            q.push_back((d_k, *d_value));
                        } else if !m.contains_key(d_kstr) {
                            m.insert(d_k.clone(), d_v.clone());
                            q.push_back((d_k, *d_value));
                        }
                    } else {
                        // d_v len > 1, trying to intersect with constraint
                        // 1. if the intersection still has len > 1, then we add it to constraint but not q
                        // 2. if the intersection only has one elem, we push it to constraint and q
                        // 3. if the intersection is empty, then it's a conflict
                        let d_kstr = d_k.as_str();
                        if !m.contains_key(d_kstr) {
                            // not in constraint, so all d_v are possible
                            // not pushing it to q
                            m.insert(d_k.clone(), d_v.clone());
                        } else {
                            // m contains d_k, performing intersection
                            let intersected_v: HashSet<_> =
                                d_v.intersection(&m[d_kstr]).cloned().collect();
                            if intersected_v.len() == 1 {
                                // intersection yields 1 elem
                                *m.get_mut(d_kstr).unwrap() = intersected_v.clone();
                                let d_value = intersected_v.iter().next().unwrap();
                                q.push_back((d_k, *d_value));
                            } else if intersected_v.len() > 1 {
                                // intersection produces multiple elems
                                *m.get_mut(d_kstr).unwrap() = intersected_v.clone();
                            } else {
                                // intersection produces empty set
                                return Ok(None);
                            }
                        }
                    }
                }
            }
        }

        Ok(Some(m))
    }

    pub fn propagate_remt(
        &self,
        func_id: usize,
        params: &Vec<Param>,
        index: usize,
        m_constraints: &HashMap<String, HashSet<i8>>,
    ) -> Result<Vec<HashMap<String, HashSet<i8>>>, Box<dyn Error>> {
        // if params.len() > 300 {

        // }
        let mut ret: Vec<HashMap<String, HashSet<i8>>> = vec![];

        let bfs_switch = true;

        // construct the solution space for current index
        let mut dim_list: Vec<i8>;
        let param_name = params[index].name.as_str();
        if m_constraints.contains_key(param_name) {
            debug!(
                "range contraints: {} -> {}",
                params[index]
                    .param_type
                    .dimensions
                    .as_ref()
                    .unwrap_or(&vec![])
                    .len()
                    + 1,
                m_constraints[param_name].len()
            );
            dim_list = m_constraints[param_name].iter().cloned().collect();
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
                        println!(
                            "remt! ({}, {}) @ index {}, m.len() = {}",
                            params[index].name.as_str(),
                            d,
                            index,
                            m_constraints.len(),
                        );

                        let bfs_result = self
                            .propagate_bfs(func_id, param_name, *d, m_constraints)
                            .unwrap();
                        if bfs_result.is_none() {
                            println!(" > bfs returns conflict");
                            return None;
                        }
                        let result = bfs_result.unwrap();
                        // if params.len() > 300 {
                            println!("- result += {:?}", result);
                        // }
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
                    println!(
                        "remt ({}, {}) @ index {}, m.len() = {}",
                        params[index].name.as_str(),
                        d,
                        index,
                        m_constraints.len(),
                    );

                    let bfs_result = self
                        .propagate_bfs(func_id, param_name, *d, m_constraints)
                        .unwrap();
                    if bfs_result.is_none() {
                        println!(" > bfs returns conflict");
                        return vec![];
                    }
                    let m = bfs_result.unwrap();

                    // if params.len() > 300 {
                    let mc = m.clone();
                    println!(" > {:?}", mc);
                    // print!(" :");
                    // for p in params.iter() {
                    //     if mc.contains_key(p.name.as_str()) {
                    //         print!("\"{}\": {:?}, ", p.name.as_str(), mc[p.name.as_str()]);
                    //     }
                    // }
                    // println!();
                    // }
                    let sub_res = self.propagate_remt(func_id, params, index + 1, &m).unwrap();
                    let mut sub_ret: Vec<HashMap<String, HashSet<i8>>> = vec![];
                    for ssr in sub_res {
                        let mut m_copied = m.clone();
                        println!(" intersecting\n  |{:?}\n  |{:?}", m_copied, ssr);
                        let suc = Self::intersect_with(&mut m_copied, &ssr);
                        if suc {
                            println!("  |=> {:?}", m_copied);
                            sub_ret.push(m_copied);
                        } else {
                            println!("  |=> Failed");
                        }
                    }
                    return sub_ret;
                })
                .collect::<Vec<HashMap<String, HashSet<i8>>>>(),
        );

        Ok(ret)
    }

    fn intersect_with(
        a: &mut HashMap<String, HashSet<i8>>,
        b: &HashMap<String, HashSet<i8>>,
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
                a.insert(k.to_string(), v.clone());
            }
        }

        true
    }

    pub fn derive(
        &self,
        func_id: usize,
        inst_id: usize,
        var_name: &str,
        split: i8,
    ) -> Result<Vec<(HashMap<String, i8>, usize)>, Box<dyn Error>> {
        let inst_derive = Derivation::derive(&self.derive, func_id, inst_id)?;
        // println!("inst_derive original: {:?}", inst_derive);
        let mut result: Vec<(HashMap<String, i8>, usize)> = vec![];
        inst_derive.iter().enumerate().for_each(|(i, d)| {
            if d.contains_key(var_name) && d[var_name] == split {
                // NOTE: we depend on the order of solution being constant here
                result.push((d.clone(), i));
            }
        });
        // println!("derive({}, {}, {}, {}) returning {} out of {} results", func_id, inst_id, var_name, split, result.len(), inst_derive.len());
        Ok(result)
    }
}