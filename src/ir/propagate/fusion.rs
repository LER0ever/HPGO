use crate::ir::derive::DeriveCache;
use crate::ir::error::DeriveError::*;
use crate::ir::hlo_ast::InstPos;
use crate::ir::propagate::ast_propagate::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::time::{Duration, Instant};

impl Context {
    pub fn get_fusion_list(&self, func_id: usize) -> Result<Vec<(InstPos, usize)>, Box<dyn Error>> {
        let now = Instant::now();
        let f = &self.ast.functions[func_id];
        let result: Vec<(InstPos, usize)> = f
            .body
            .iter()
            .map(|inst| {
                if inst.function.name == "fusion" {
                    let fn_name = inst.get_meta_str("calls").unwrap();
                    let func_id = self.ast.func_id[&fn_name];
                    let inst_pos = self.ast.inst_pos[inst];
                    Some((inst_pos, func_id))
                } else {
                    None
                }
            })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();
        println!("[fusion]\t fusion list returning {} entries", result.len());
        println!(
            "[fusion]\t Extract Fusion Fn List... {}ms",
            now.elapsed().as_millis()
        );
        Ok(result)
    }

    pub fn update_fusion_derive_cache(
        &mut self,
        func_id: usize,
    ) -> Result<DeriveCache, Box<dyn Error>> {
        let fusion_list = self.get_fusion_list(func_id)?;
        let mut now = Instant::now();
        let mut derive_patch: DeriveCache = HashMap::new();
        derive_patch.par_extend(fusion_list.par_iter().map(|(inst_pos, func_id)| {
            let f = &self.ast.functions[*func_id];
            let inst = &self.ast.functions[inst_pos.0].body[inst_pos.1];
            let inst_params_option = &inst.function.params;
            assert_eq!(inst_params_option.is_some(), true);
            let inst_params = inst_params_option.as_ref().unwrap();
            let fn_params = &f.params;
            assert_eq!(inst_params.len(), fn_params.len());
            let fn_return_var = &f.body[f.body.len() - 1].var_name;
            let inst_return_var = &inst.var_name;

            // println!("processing fusion fn {}", inst
            //     .get_meta_str("calls").unwrap());

            let p_result = self.propagate_fn(*func_id).unwrap();
            let mut flattened_result: Vec<HashMap<String, i8>> = vec![];
            for m in p_result {
                // debug!("processing map: {:?}", m);
                let mut flattened_map: HashMap<String, i8> = HashMap::new();
                for (k, v) in m {
                    if &k == fn_return_var {
                        flattened_map.insert(inst_return_var.to_string(), v.iter().cloned().next().unwrap());
                    } else {
                        for (i, p) in f.params.iter().enumerate() {
                            if k.contains(&p.name) {
                                if v.len() > 1 {
                                    println!("[fusion]\t resulting set has more than 1 element, we are losing solution space: inst = {} | k = {}, v = {:?}", inst
                                        .get_meta_str("calls").unwrap(), k, v);
                                }
                                flattened_map.insert(
                                    (&inst.function.params.as_ref().unwrap()[i].name).to_string(),
                                    v.iter().cloned().next().unwrap(),
                                );
                            }
                        }
                    }
                }
                flattened_result.push(flattened_map);
            }
            ((inst_pos.0, inst_pos.1), flattened_result)
        }).collect::<Vec<(InstPos, Vec<HashMap<String, i8>>)>>());
        println!(
            "[fusion]\t Compute Fusion Derivation... {}ms",
            now.elapsed().as_millis()
        );

        println!("merging with derive_cache...");
        let result = derive_patch.clone();
        now = Instant::now();

        for (inst_pos, m) in derive_patch {
            if self.derive.contains_key(&inst_pos) {
                *self.derive.get_mut(&inst_pos).unwrap() = m;
            } else {
                self.derive.insert(inst_pos, m);
            }
        }

        println!(
            "[fusion]\t Merge fusion derivation with derive_cache... {}ms",
            now.elapsed().as_millis()
        );

        Ok(result)
    }
}
