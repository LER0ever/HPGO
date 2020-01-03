#![allow(non_snake_case)]

extern crate HPGO;
extern crate rayon;
use rayon::prelude::*;
use std::collections::BTreeSet;
use HPGO::analysis::*;
use HPGO::environment::*;
use HPGO::input::*;
use HPGO::model::*;
use HPGO::orchestration::*;
use HPGO::parallelism::*;

fn test_xlnet_speedup_at_all_bs() {
    // GBS
    let mut gbs = vec![1, 2, 4, 8, 16, 32, 64];
    for i in 1..((2048 - 64) / 64) {
        gbs.push(64 + i * 64);
    }
    // Compute Max Batch Size in Parallel
    let res: Vec<_> = gbs
        .par_iter()
        .map(|(gbs)| {
            // Construct Model
            let tgi: torch_graph::TorchGraphImporter = ModelImporter::new();
            let result = tgi.ImportFrom(&["./profiles/", "xlnet", "/graph.txt"].join(""));
            let (perf, states) = (result.0.unwrap(), result.1.unwrap());
            let mut model = model::Model::new_from_model_perf(perf, states, 1, *gbs);
            model.set_optimizer_memory_scaling(3);
            model.set_peak_activation_per_batch(3942774528.0);
            model.set_min_microbatch_size(1);
            // Construct Devices
            let d16 = device::Devices::new(16, vec![8, 16]);
            // DP Speedups
            let dp_speedup = data_parallel::dp_speedup(&d16, &model);
            // let dp_p3_speedup = data_parallel::dp_p3_speedup(&d16, &model);
            let dp_cur_ga_p3_speedup = gradient_accumulation::dp_cur_ga_p3_speedup(&d16, &model);
            let dp_opt_ga_p3_speedup = gradient_accumulation::dp_opt_ga_p3_speedup(&d16, &model);
            // Hybrid Parallelism Speedups
            let mut c = orchestrate::SyncConductor::new_from_model_device(model, d16);
            c.orchestrate();
            let mut pipeline_speedup = 0.0;
            let mut pipeline_stages: Vec<(u32, u32, u32, BTreeSet<u32>)> = vec![];
            for s in c.res {
                if s.speedup > pipeline_speedup {
                    pipeline_speedup = s.speedup;
                    pipeline_stages = s.stages;
                }
            }
            // NOTE: the following block is not thread-safe
            // let pipeline_speedup = c.res.par_iter().map(|s| {
            //     s.speedup
            // }).fold(0./0., f64::max)
            // return gbs and all speedups
            (
                gbs,
                (
                    dp_speedup,
                    dp_cur_ga_p3_speedup,
                    dp_opt_ga_p3_speedup,
                    pipeline_speedup,
                    pipeline_stages,
                ),
            )
        })
        .collect();

    for i in res {
        println!(
            "{} | {} | {} | {} | {} | {:?}",
            i.0,
            (i.1).0,
            (i.1).1,
            (i.1).2,
            (i.1).3,
            (i.1).4
        );
    }
}
fn main() {
    test_xlnet_speedup_at_all_bs();
}
