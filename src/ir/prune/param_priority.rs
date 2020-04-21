use crate::ir::propagate::ast_propagate::*;
use crate::ir::hlo_ast::{HLORoot, Param};
use rayon::prelude::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub fn prioritized_params(ctx: &Context, func_id: usize, params: &Vec<Param>) -> Vec<Param> {
    let now = Instant::now();
    let mut p_values: Vec<(usize, i32)> = params.iter().enumerate().map(|(i, p)| {
        let bfs_result = ctx.propagate_bfs(func_id, &p.name, -1, &HashMap::new(), false).unwrap();
        if bfs_result.is_none() {
            return (i, -1);
        } else {
            return (i, bfs_result.unwrap().len() as i32);
        }
    }).collect();

    // sort p_values in decreasing order
    p_values.sort_by_key(|x| x.1);
    p_values.reverse();

    let mut p_vec = vec![];
    for p in p_values {
        p_vec.push(params[p.0].clone());
    }

    println!(
        "[pruning]\t Reorder Input Params... {}ms",
        now.elapsed().as_millis()
    );

    p_vec
}
