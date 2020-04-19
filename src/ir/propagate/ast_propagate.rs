#![feature(atomic_min_max)]

use crate::ir::derive::{Derivation, DeriveCache};
use crate::ir::error::PropagationError::*;
use crate::ir::hlo_ast::{HLORoot, Param};
use log::debug;
use petgraph::prelude::*;
use pyo3::prelude::*;
use rayon::prelude::*;
use stacker;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::io;
use std::io::Write;
use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

const DEBUG: bool = false;
static CUR_MAX_PROGRESS: AtomicUsize = AtomicUsize::new(0);

#[pyclass]
#[derive(Clone, Debug, Default)]
pub struct Context {
    pub ast: HLORoot,
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

    pub fn propagate_fn(
        &self,
        func_id: usize,
    ) -> Result<Vec<HashMap<String, HashSet<i8>>>, Box<dyn Error>> {
        let f = &self.ast.functions[func_id];
        let params = f.params.clone();
        let result = self.propagate_remt(func_id, &params, 0, &HashMap::new(), vec![], false)?;
        Ok(result)
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
        debug: bool,
    ) -> Result<Option<(HashMap<String, HashSet<i8>>)>, Box<dyn Error>> {
        // Option<(determined set, undetermined set)>
        // make a copy of constraint map, pending return.
        let mut m = constraints.clone();
        // NOTE: check compliance with constraint
        if m.contains_key(p_name) && !m[p_name].contains(&split) {
            if DEBUG || debug {
                println!("input param and split don't comply with constraints");
            }
            return Ok(None); // conflict with constraint
        } else if m.contains_key(p_name) && m[p_name].contains(&split) && m[p_name].len() == 1 {
            // NOTE: the constraints is already minimal, we've explored this param.
            return Ok(Some(m));
        }
        assert_eq!(!m.contains_key(p_name) || m[p_name].contains(&split), true);
        // NOTE: since we fixed p_name and split, we update the constraint set for p_name
        if m.contains_key(p_name) {
            *m.get_mut(p_name).unwrap() = [split].iter().cloned().collect();
        } else {
            m.insert(p_name.to_string(), [split].iter().cloned().collect());
        }

        let f = &self.ast.functions[func_id];
        assert_eq!(self.ast.var_pos[p_name].0, func_id);

        // NOTE: bfs start
        //
        let mut q: VecDeque<(String, i8)> = VecDeque::new();
        let mut visited: HashSet<(String, i8)> = HashSet::new();

        q.push_back((p_name.to_string(), split));
        visited.insert((p_name.to_string(), split));
        while !q.is_empty() {
            let (v, s) = q.pop_front().unwrap();
            let v_pos = &self.ast.var_pos[&v].1;
            if DEBUG || debug {
                println!(
                    "\t[bfs]\t ({}, {}) exploring {} positions",
                    v,
                    s,
                    v_pos.len()
                );
            }
            // NOTE: iterate over all positions of the variable
            for vp in v_pos {
                // if BFS_DEBUG {
                //     println!("exploring inst ({}, {})", func_id, vp);
                // }
                let v_derive: Vec<(HashMap<String, i8>, usize)> =
                    self.derive(func_id, *vp, &v, s)?;
                if v_derive.len() == 0
                    && (self.ast.functions[func_id].body[*vp].function.name != "tuple"
                        && self.ast.functions[func_id].body[*vp].function.name != "rng"
                        && self.ast.functions[func_id].body[*vp].function.name != "constant"
                        && self.ast.functions[func_id].body[*vp].function.name != "copy"
                        && self.ast.functions[func_id].body[*vp].function.name != "iota"
                        && self.ast.functions[func_id].body[*vp].function.name != "parameter")
                {
                    if DEBUG || debug {
                        println!(
                            "conflict while bfs({},{})! inst @ {} not containing (v, s)",
                            v, s, vp
                        );
                        println!(
                            "enumeration for inst @ {}: \n{:?}",
                            vp,
                            Derivation::derive(&self.derive, func_id, *vp)?
                        );
                    }
                    return Ok(None);
                }
                // if DEBUG || debug {
                //     println!("\t\tv_derive for {}, {} @ {} has {} entries", v, s, vp, v_derive.len());
                // }
                let mut derive_aggregated: HashMap<String, HashSet<i8>> = HashMap::new();
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
                if self.ast.functions[func_id].body[*vp].function.name == "fusion"
                    && self.ast.functions[func_id].body[*vp].get_meta_str("calls")?
                        == "%fused_computation.4970.clone"
                {
                    println!("fusion 4970, derive ag: {:?}", derive_aggregated);
                }

                for (d_k, d_v) in derive_aggregated {
                    if d_k == v {
                        continue;
                    }
                    /*
                    if d_k == v && d_v.contains(&s){
                        // break;
                        continue;
                        // NOTE: this means that we've
                    }
                    if d_k == v && !d_v.contains(&s) {
                        if DEBUG || debug {
                            println!(
                                "conflict while bfs({},{})! d_k = v, but s not in d_v {:?}",
                                v, s, d_v
                            );
                        }
                        return Ok(None);
                    }
                    */

                    // if BFS_DEBUG {
                    //     println!("d_k: {}, d_v: {:?}", d_k, d_v );
                    // }

                    if d_v.len() == 1 {
                        let d_value = d_v.iter().next().unwrap();
                        let d_kstr = d_k.as_str();
                        if m.contains_key(d_kstr) && !m[d_kstr].contains(d_value) {
                            if DEBUG || debug {
                                println!(
                                    "conflict while bfs({},{})! for {}: {} not in {:?}",
                                    v, s, d_kstr, d_value, m[d_kstr]
                                );
                            }

                            // split into conflicting dimension
                            return Ok(None);
                        } else if m.contains_key(d_kstr)
                            && m[d_kstr].contains(d_value)
                            && m[d_kstr].len() == 1
                        {
                            // split into the same dimension, usually means we've explored this var+split combination
                            // TODO: check if we need to add this back into queue
                            // println!("TODO branch triggered");
                        } else if m.contains_key(d_kstr)
                            && m[d_kstr].contains(d_value)
                            && m[d_kstr].len() > 1
                        {
                            // split into compliant dimension, need to shrink the set to single element
                            *m.get_mut(d_kstr).unwrap() = d_v.clone();

                            if !visited.contains(&(d_k.clone(), *d_value)) {
                                if DEBUG || debug {
                                    println!(
                                        "\t[bfs]\t\t adding ({},{}) to q, from fn {}, reduce m to 1",
                                        d_k, *d_value, self.ast.functions[func_id].body[*vp].function.name
                                    );
                                }
                                visited.insert((d_k.clone(), *d_value));
                                q.push_back((d_k, *d_value));
                            }
                        } else if !m.contains_key(d_kstr) {
                            m.insert(d_k.clone(), d_v.clone());

                            if !visited.contains(&(d_k.clone(), *d_value)) {
                                if DEBUG || debug {
                                    println!(
                                        "\t[bfs]\t\t adding ({},{}) to q, from fn {}, new entry m to 1",
                                        d_k, *d_value, self.ast.functions[func_id].body[*vp].function.name
                                    );
                                }
                                visited.insert((d_k.clone(), *d_value));
                                q.push_back((d_k, *d_value));
                            }
                        } else {
                            println!("[\t[bfs]\t Unhandled branch !");
                        }
                    } else {
                        // NOTE: d_v len > 1, trying to intersect with constraint
                        // 1. if the intersection still has len > 1, then we add it to constraint but not q
                        // 2. if the intersection only has one elem, we push it to constraint and q
                        // 3. if the intersection is empty, then it's a conflict
                        let d_kstr = d_k.as_str();
                        if !m.contains_key(d_kstr) {
                            // not in constraint, so all d_v are possible
                            // not pushing it to q
                            m.insert(d_k.clone(), d_v.clone());
                            if DEBUG || debug {
                                println!(
                                    "\t[bfs]\t\t from fn {}, map new entry {}: {:?}",
                                    self.ast.functions[func_id].body[*vp].function.name, d_k, d_v
                                );
                            }
                        } else {
                            // m contains d_k, performing intersection
                            let intersected_v: HashSet<_> =
                                d_v.intersection(&m[d_kstr]).cloned().collect();
                            if intersected_v.len() == 1 {
                                // intersection yields 1 elem
                                *m.get_mut(d_kstr).unwrap() = intersected_v.clone();
                                let d_value = intersected_v.iter().next().unwrap();

                                if !visited.contains(&(d_k.clone(), *d_value)) {
                                    if DEBUG || debug {
                                        println!("\t[bfs]\t\t adding ({},{}) to q, from fn {}, reduce to 1 (intersection)", d_k, *d_value, self.ast.functions[func_id].body[*vp].function.name);
                                    }
                                    visited.insert((d_k.clone(), *d_value));
                                    q.push_back((d_k, *d_value));
                                }
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

    pub fn propagate_iemt(
        &self,
        func_id: usize,
        params: &Vec<Param>,
        index: usize,
        m_constraints: &HashMap<String, HashSet<i8>>,
    ) -> Result<Vec<HashMap<String, HashSet<i8>>>, Box<dyn Error>> {
        // NOTE: stack contains (var index within params, var split), constraints map
        // let stack: Vec<(usize, usize), HashMap<String, HashSet<i8>>> = vec![];

        unimplemented!()
    }

    pub fn propagate_remt(
        &self,
        func_id: usize,
        params: &Vec<Param>,
        index: usize,
        m_constraints: &HashMap<String, HashSet<i8>>,
        debug_chain: Vec<i8>,
        main_task: bool,
    ) -> Result<Vec<HashMap<String, HashSet<i8>>>, Box<dyn Error>> {
        // if params.len() > 300 {
        //     println!("remt @ {}", index);
        // }
        let mut ret: Vec<HashMap<String, HashSet<i8>>> = vec![];

        let bfs_switch = true;
        let mut debug = false;
        // let bfs_debug = params.len() > 150;
        let bfs_debug = false;

        // Max Progress Pruning for main_task
        if main_task {
            let mut cur_rep = 0usize;
            let cur_max = CUR_MAX_PROGRESS.load(Ordering::SeqCst);
            for s in debug_chain.iter() {
                if *s == -1 {
                    cur_rep += 1;
                }
            }
            let max_possible_split = params.len() - cur_rep;
            if max_possible_split < cur_max {
                print!(".");
                io::stdout().flush().unwrap();
                return Ok(vec![]);
            }
        }

        // construct the solution space for current index
        let mut dim_list: Vec<i8>;
        let param_name = params[index].name.as_str();
        if m_constraints.contains_key(param_name) {
            debug!(
                "range constraints: {} -> {}",
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

        // NOTE: hard code first two var
        // if params.len() > 300 {
        //     if index == 0 {
        //         dim_list = vec![0];
        //     } else if index == 1 {
        //         dim_list = vec![-1];
        //     } else if index == 2 {
        //         dim_list = vec![1];
        //     }
        // }

        if index + 1 == params.len() {
            ret.par_extend(
                dim_list
                    .par_iter()
                    .map(|d| {
                        if DEBUG || debug {
                            println!(
                                "remt! ({}, {}) @ index {}, m.len() = {}",
                                params[index].name.as_str(),
                                d,
                                index,
                                m_constraints.len(),
                            );
                        }
                        let mut chain = debug_chain.clone();
                        chain.push(*d);

                        let bfs_result = self
                            .propagate_bfs(func_id, param_name, *d, m_constraints, bfs_debug)
                            .unwrap();
                        if bfs_result.is_none() {
                            if DEBUG || debug {
                                println!(" > bfs returns conflict");
                            } else if main_task {
                                // println!("X Conflict @ index {}:{}\n{:?}", index, d, chain);
                                print!("X");
                                io::stdout().flush().unwrap();
                            }

                            return None;
                        }
                        let result = bfs_result.unwrap();
                        if DEBUG || debug {
                            println!("- result += {:?}", result);
                        } else if main_task {
                            let mut res: Vec<i8> = vec![];
                            let mut split_progress = 0usize;
                            for p in params.iter() {
                                if result.contains_key(&p.name) {
                                    let split =
                                        result[&p.name].clone().iter().cloned().next().unwrap();
                                    if split != -1 {
                                        split_progress += 1;
                                    }
                                    res.push(split);
                                }
                            }
                            CUR_MAX_PROGRESS.fetch_max(split_progress, Ordering::SeqCst);
                            println!("\nO solution: {}\n{:?}", split_progress, res);
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
                    if DEBUG || debug {
                        println!(
                            "remt ({}, {}) @ index {}, m.len() = {}",
                            params[index].name.as_str(),
                            d,
                            index,
                            m_constraints.len(),
                        );
                    }
                    let mut chain = debug_chain.clone();
                    chain.push(*d);

                    let bfs_result = self
                        .propagate_bfs(func_id, param_name, *d, m_constraints, bfs_debug)
                        .unwrap();
                    if bfs_result.is_none() {
                        if DEBUG || debug {
                            println!(" > bfs returns conflict");
                        } else if main_task {
                            print!("x");
                            io::stdout().flush().unwrap();
                        }
                        return vec![];
                    }
                    let m = bfs_result.unwrap();

                    // if params.len() > 300 {
                    let mc = m.clone();
                    if DEBUG || debug {
                        println!(" > {:?}", mc);
                    }
                    let sub_res = stacker::maybe_grow(32 * 1024, 1024 * 1024, || {
                        self.propagate_remt(func_id, params, index + 1, &m, chain, main_task)
                            .unwrap()
                    });
                    let mut sub_ret: Vec<HashMap<String, HashSet<i8>>> = vec![];
                    for ssr in sub_res {
                        let mut m_copied = m.clone();
                        if DEBUG || debug {
                            println!(" intersecting\n  |{:?}\n  |{:?}", m_copied, ssr);
                        }
                        let suc = Self::intersect_with(&mut m_copied, &ssr);
                        if suc {
                            if DEBUG || debug {
                                println!("  |=> {:?}", m_copied);
                            }

                            sub_ret.push(m_copied);
                        } else {
                            if DEBUG || debug {
                                println!("  |=> Failed");
                            }
                        }
                    }
                    return sub_ret;
                })
                .collect::<Vec<HashMap<String, HashSet<i8>>>>(),
        );

        Ok(ret)
    }

    pub fn propagate_remtp(
        &self,
        func_id: usize,
        params: &Vec<Param>,
        index: usize,
        m_constraints: &HashMap<String, HashSet<i8>>,
        debug_chain: Vec<i8>,
        main_task: bool,
    ) -> Result<(), Box<dyn Error>> {
        // if params.len() > 300 {
        //     println!("remt @ {}", index);
        // }
        let bfs_switch = true;
        let mut debug = false;
        // let bfs_debug = params.len() > 150;
        let bfs_debug = false;

        // Max Progress Pruning for main_task
        if main_task {
            let mut cur_rep = 0usize;
            let cur_max = CUR_MAX_PROGRESS.load(Ordering::SeqCst);
            for s in debug_chain.iter() {
                if *s == -1 {
                    cur_rep += 1;
                }
            }
            let max_possible_split = params.len() - cur_rep;
            if max_possible_split < cur_max {
                print!(".");
                io::stdout().flush().unwrap();
                return Ok(());
            }
        }

        // construct the solution space for current index
        let mut dim_list: Vec<i8>;
        let param_name = params[index].name.as_str();
        if m_constraints.contains_key(param_name) {
            debug!(
                "range constraints: {} -> {}",
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

        // NOTE: hard code first two var
        if main_task {
            if index == 0 {
                println!("using hardcoded solution for index {}", index);
                dim_list = vec![0];
            } else if index == 1 {
                println!("using hardcoded solution for index {}", index);
                dim_list = vec![-1];
            } else if index == 2 {
                println!("using hardcoded solution for index {}", index);
                dim_list = vec![1];
            }
        }

        if index + 1 == params.len() {
            dim_list.par_iter().for_each(|d| {
                if DEBUG || debug {
                    println!(
                        "remt! ({}, {}) @ index {}, m.len() = {}",
                        params[index].name.as_str(),
                        d,
                        index,
                        m_constraints.len(),
                    );
                }
                let mut chain = debug_chain.clone();
                chain.push(*d);

                let bfs_result = self
                    .propagate_bfs(func_id, param_name, *d, m_constraints, bfs_debug)
                    .unwrap();
                if bfs_result.is_none() {
                    if DEBUG || debug {
                        println!(" > bfs returns conflict");
                    } else if main_task {
                        // println!("X Conflict @ index {}:{}\n{:?}", index, d, chain);
                        print!("X");
                        io::stdout().flush().unwrap();
                    }
                    return;
                }
                let result = bfs_result.unwrap();
                if DEBUG || debug {
                    println!("- result += {:?}", result);
                } else if main_task {
                    let mut res: Vec<i8> = vec![];
                    let mut split_progress = 0usize;
                    for p in params.iter() {
                        if result.contains_key(&p.name) {
                            let split = result[&p.name].clone().iter().cloned().next().unwrap();
                            if split != -1 {
                                split_progress += 1;
                            }
                            res.push(split);
                        }
                    }
                    CUR_MAX_PROGRESS.fetch_max(split_progress, Ordering::SeqCst);
                    println!("\nO solution: {}\n{:?}", split_progress, res);
                }
                return;
                // ret.push(result);
            });

            return Ok(());
        }

        // recursive enumeration
        dim_list.par_iter().for_each(|d| {
            let mut chain = debug_chain.clone();
            chain.push(*d);

            let bfs_result = self
                .propagate_bfs(func_id, param_name, *d, m_constraints, bfs_debug)
                .unwrap();
            if bfs_result.is_none() {
                if DEBUG || debug {
                    println!(" > bfs returns conflict");
                } else if main_task {
                    print!("x");
                    io::stdout().flush().unwrap();
                }
                return;
            }
            let m = bfs_result.unwrap();

            stacker::maybe_grow(32 * 1024, 1024 * 1024, || {
                self.propagate_remtp(func_id, params, index + 1, &m, chain, main_task)
                    .unwrap()
            });
            return;
        });

        Ok(())
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
        // if func_id == 10206 && inst_id == 29 {
        //     println!("inst_derive original: {:?}", inst_derive);
        // }

        let mut result: Vec<(HashMap<String, i8>, usize)> = vec![];
        for (i, d) in inst_derive.iter().enumerate() {
            if d.contains_key(var_name) && d[var_name] == split {
                // NOTE: we depend on the order of solution being constant here
                result.push((d.clone(), i));
            }
        }

        // if func_id == 10206 && inst_id == 29 {
        //     println!("derive({}, {}, {}, {}) returning {} out of {} results", func_id, inst_id, var_name, split, result.len(), inst_derive.len());
        // }

        Ok(result)
    }
}
