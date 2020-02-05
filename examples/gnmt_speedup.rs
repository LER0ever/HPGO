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

fn test_bert_speedup_at_all_bs() {
    // GBS
    let mut gbs = vec![1, 2, 4, 8, 16];
    for i in 1..((256 - 16) / 8) + 1 {
        gbs.push(16 + i * 8);
    }

    // Compute Max Batch Size in Parallel
    let res: Vec<_> = gbs
        .par_iter()
        .map(|(gbs)| {
            // Construct Model
            let tgi: torch_graph::TorchGraphImporter = ModelImporter::new();
            let result =
                tgi.ImportFrom(&["./profiles/", "gnmt_32_simplified", "/graph.txt"].join(""));
            let (perf, states) = (result.0.unwrap(), result.1.unwrap());
            let mut model = model::Model::new_from_model_perf(perf, states, 1, *gbs);
            model.optimizer_memory_scaling = 3;
            model.peak_activation_per_batch = 3942774528.0 * 1.5; // don't have data for XLNet-36, use 24 * 1.5
            model.min_micro_batch_size = 1;
            // Construct Devices
            let d16 = device::Devices::new(16, vec![8, 16]);

            // DP Speedups
            let dp_speedup = data_parallel::dp_speedup(&d16, &model);
            // let dp_p3_speedup = data_parallel::dp_p3_speedup(&d16, &model);
            let dp_ga_p3_speedup = gradient_accumulation::dp_cur_ga_p3_speedup(&d16, &model);
            let dp_ga_inner_overlap_speedup =
                gradient_accumulation::dp_cur_ga_inner_overlap_speedup(&d16, &model);

            let model_cloned = model.clone();
            let d16_cloned = d16.clone();

            let mut c = orchestrate_async::AsyncOrchestrate::new_from_model_device(model, d16);
            // Straight Pipeline
            let res_straight = c.plan_for(16, true);
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

            let pipeline_recursive_speedup = sync_pipeline::sync_pipeline_speedup_recursive(
                &d16_cloned,
                &model_cloned,
                best_hp.rp,
                pipeline_stages.clone(),
            );

            // return gbs and all speedups
            (
                gbs,
                (
                    dp_speedup,
                    dp_ga_p3_speedup,
                    dp_ga_inner_overlap_speedup,
                    straight_speedup,
                    pipeline_speedup,
                    pipeline_recursive_speedup,
                    pipeline_stages,
                ),
            )
        })
        .collect();

    println!("Global Batch Size, DP No Overlap, DP+P3, DP+Normal Overlap, Straight Speedup, Best Hybrid Speedup, Best Hybrid Speedup (recursive) | Best Hybrid Solution");
    for i in res {
        println!(
            "{}, {}, {}, {}, {}, {}, {} | {:?}",
            i.0,
            (i.1).0,
            (i.1).1,
            (i.1).2,
            (i.1).3,
            (i.1).4,
            (i.1).5,
            (i.1).6,
            // (i.1).7,
        );
    }
}
fn main() {
    test_bert_speedup_at_all_bs();
}
