#![allow(non_snake_case)]

extern crate HPGO;
extern crate ordered_float;
extern crate rayon;
use ordered_float::OrderedFloat;
use rayon::prelude::*;

use std::collections::BTreeSet;

use HPGO::environment::*;
use HPGO::input::*;
use HPGO::layerwise::model::*;
use HPGO::layerwise::orchestration::*;
use HPGO::layerwise::parallelism::*;

fn test_vgg_speedup_at_all_bs() {
    // GBS
    let mut gbs = vec![32, 64];
    for i in 1..((4096 - 64) / 64) + 1 {
        gbs.push(64 + i * 64);
    }

    // Compute Max Batch Size in Parallel
    let res: Vec<_> = gbs
        .par_iter()
        .map(|gbs| {
            // Construct Model
            let tgi: torch_graph::TorchGraphImporter = LayerwiseModelImporter::new();
            let result = tgi.ImportFrom(&["./profiles/", "vgg19", "/graph.txt"].join(""));
            let (perf, states) = (result.0.unwrap(), result.1.unwrap());
            let mut model = model::Model::new_from_model_perf(perf, states, 32, *gbs);
            model.optimizer_memory_scaling = 2;
            model.min_micro_batch_size = 32;
            model.peak_activation_per_batch = 70000000.0;
            // Construct Devices
            let d16 = device::Devices::new(
                16,
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            );

            // DP Speedups
            let dp_speedup = data_parallel::dp_speedup(&d16, &model);
            // let dp_p3_speedup = data_parallel::dp_p3_speedup(&d16, &model);
            let dp_ga_p3_speedup = gradient_accumulation::dp_cur_ga_p3_speedup(&d16, &model);
            let dp_ga_inner_overlap_speedup =
                gradient_accumulation::dp_cur_ga_inner_overlap_speedup(&d16, &model);

            // Hybrid Parallelism Speedups
            let mut c = orchestrate_async::AsyncOrchestrate::new_from_model_device(model, d16);
            c.orchestrate();
            let mut pipeline_speedup = 0.0;
            let mut pipeline_stages: Vec<(u32, u32, u32, BTreeSet<u32>)> = vec![];

            let best_hp = c
                .res
                .into_par_iter()
                .max_by_key(|r| OrderedFloat(r.speedup))
                .unwrap();
            pipeline_speedup = best_hp.speedup;
            pipeline_stages = best_hp.stages;

            // return gbs and all speedups
            (
                gbs,
                (
                    dp_speedup,
                    dp_ga_p3_speedup,
                    dp_ga_inner_overlap_speedup,
                    pipeline_speedup,
                    pipeline_stages,
                ),
            )
        })
        .collect();

    println!("Global Batch Size, DP No Overlap, DP+P3, DP+Normal Overlap, Best Hybrid Speedup | Best Hybrid Solution");
    for i in res {
        println!(
            "{}, {}, {}, {}, {} | {:?}",
            i.0,
            (i.1).0,
            (i.1).1,
            (i.1).2,
            (i.1).3,
            (i.1).4,
            // (i.1).7,
        );
    }
}
fn main() {
    test_vgg_speedup_at_all_bs();
}
