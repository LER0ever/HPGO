#![allow(non_snake_case)]

extern crate HPGO;
extern crate ordered_float;
extern crate rayon;
use ordered_float::OrderedFloat;
use rayon::prelude::*;
use std::cmp::min;
use std::collections::BTreeSet;
use HPGO::analysis::*;
use HPGO::environment::*;
use HPGO::input::*;
use HPGO::model::*;
use HPGO::orchestration::*;
use HPGO::parallelism::*;

fn test_gnmt_speedup_at_all_bs() {
    // GBS
    let gbs = 512;
    let mut d: Vec<Vec<u32>> = vec![];
    for i in 2..16 + 1 {
        let mut cur_d: Vec<u32> = vec![i];
        if i > 24 {
            cur_d.insert(0, 24);
        }
        if i > 16 {
            cur_d.insert(0, 16);
        }
        if i > 8 {
            cur_d.insert(0, 8);
        }
        d.push(cur_d);
    }

    println!("D Matrix: {:?}", d);

    // Compute Max Batch Size in Parallel
    let res: Vec<_> = d
        .par_iter()
        .map(|(cur_d)| {
            // Construct Model
            let tgi: torch_graph::TorchGraphImporter = ModelImporter::new();
            let result = tgi.ImportFrom(&["./profiles/", "gnmt_large", "/graph.txt"].join(""));
            let (perf, states) = (result.0.unwrap(), result.1.unwrap());
            let mut model = model::Model::new_from_model_perf(perf, states, 128, gbs);
            model.optimizer_memory_scaling = 3;
            model.peak_activation_per_batch = 100000000.0; // don't have data for XLNet-36, use 24 * 1.5
            model.min_micro_batch_size = 64;
            // Construct Devices
            let d16 = device::Devices::new(cur_d[cur_d.len() - 1], cur_d.to_vec());

            // DP Speedups
            let dp_speedup = data_parallel::dp_speedup(&d16, &model);
            // let dp_p3_speedup = data_parallel::dp_p3_speedup(&d16, &model);
            let dp_ga_p3_speedup = gradient_accumulation::dp_cur_ga_p3_speedup(&d16, &model);
            let dp_ga_inner_overlap_speedup =
                gradient_accumulation::dp_cur_ga_inner_overlap_speedup(&d16, &model);

            let mut c = orchestrate_async::AsyncOrchestrate::new_from_model_device(model, d16);
            // Straight Pipeline
            println!("Planning for {}", cur_d[cur_d.len() - 1]);
            let res_straight = c.plan_for(cur_d[cur_d.len() - 1], true);
            let straight_speedup = res_straight.speedup;

            // Hybrid Parallelism Speedups
            c.orchestrate();

            let best_hp = c
                .res
                .into_par_iter()
                .max_by_key(|r| {
                    if r.stages.len() == 1 {
                        // throw away pseudo DPs
                        OrderedFloat(0.0)
                    } else {
                        OrderedFloat(r.speedup)
                    }
                })
                .unwrap();
            let pipeline_speedup = best_hp.speedup;
            let pipeline_stages = best_hp.stages;

            // return gbs and all speedups
            (
                cur_d[cur_d.len() - 1],
                (
                    dp_speedup,
                    dp_ga_p3_speedup,
                    dp_ga_inner_overlap_speedup,
                    straight_speedup,
                    pipeline_speedup,
                    pipeline_stages,
                ),
            )
        })
        .collect();

    println!("Num of GPU, DP No Overlap, DP+P3, DP+Normal Overlap, Straight Speedup, Best Hybrid Speedup | Best Hybrid Solution");
    for i in res {
        println!(
            "{}, {}, {}, {}, {}, {} | {:?}",
            i.0,
            (i.1).0,
            (i.1).1,
            (i.1).2,
            (i.1).3,
            (i.1).4,
            (i.1).5,
            // (i.1).7,
        );
    }
}
fn main() {
    test_gnmt_speedup_at_all_bs();
}
