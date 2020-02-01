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

const VERBOSE: bool = true;

struct ModelConfig {
    gbs: Vec<u32>,    // GBS vector
    filename: String, // filename to TorchGraph txt
    optimizer_memory_scaling: u32,
    pbs: u32,
    mbs: u32,
    papb: f64,
}

fn test_speedup_at_all_bs(mc: ModelConfig, flat: bool) {
    let d: device::Devices;
    if flat {
        d = get_flat_devices();
    } else {
        d = get_hierarchical_devices();
    }

    // model at pbs
    if VERBOSE {
        println!("[main]\t Importing Model from TorchGraph...")
    }
    let tgi: torch_graph::TorchGraphImporter = ModelImporter::new();
    let result = tgi.ImportFrom(&mc.filename);
    let (perf, states) = (result.0.unwrap(), result.1.unwrap());
    if VERBOSE {
        println!("[main]\t Constructing HPGO Model...")
    }
    let mut m0 = model::Model::new_from_model_perf(perf, states, mc.pbs, mc.pbs);
    m0.optimizer_memory_scaling = mc.optimizer_memory_scaling;
    m0.min_micro_batch_size = mc.mbs;
    if mc.papb > 0.0 {
        m0.peak_activation_per_batch = mc.papb;
    }

    if VERBOSE {
        println!("[main]\t Model Import Complete. Starting Parallel Planning...")
    }

    // Compute Max Batch Size in Parallel
    let res: Vec<_> = mc
        .gbs
        .par_iter()
        .map(|(gbs)| {
            if VERBOSE {
                println!("[main]\t Planning in parallel for bs = {} ...", *gbs);
            }
            let m1 = m0.normalized_copy(*gbs);

            // DP Speedups
            let dp_speedup = data_parallel::dp_speedup(&d, &m1);
            // let dp_p3_speedup = data_parallel::dp_p3_speedup(&d16, &model);
            let dp_ga_p3_speedup = gradient_accumulation::dp_cur_ga_p3_speedup(&d, &m1);
            let dp_ga_inner_overlap_speedup =
                gradient_accumulation::dp_cur_ga_inner_overlap_speedup(&d, &m1);

            // Hybrid Parallelism Speedups
            let mut c =
                orchestrate_async::AsyncOrchestrate::new_from_model_device(m1.clone(), d.clone());
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

            if VERBOSE {
                println!("[main]\t Got all results for bs = {} ...", *gbs);
            }

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
    test_speedup_at_all_bs(get_resnet50_model_config(), false);
}

/// Data Area

// Seps Array for Flat and Hierarchical
fn get_hierarchical_devices() -> device::Devices {
    device::Devices::new(16, vec![8, 16])
}

fn get_flat_devices() -> device::Devices {
    device::Devices::new(
        16,
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
    )
}

fn get_vgg19_model_config() -> ModelConfig {
    let mut gbs = vec![32, 64];
    for i in 1..((4096 - 64) / 64) + 1 {
        gbs.push(64 + i * 64);
    }
    ModelConfig {
        gbs: gbs,
        filename: ["./profiles/", "vgg19", "/graph.txt"].join(""),
        optimizer_memory_scaling: 2,
        pbs: 32,
        mbs: 32,
        papb: 70000000.0,
    }
}

fn get_resnet50_model_config() -> ModelConfig {
    let mut gbs = vec![32, 64];
    for i in 1..((4096 - 64) / 64) + 1 {
        gbs.push(64 + i * 64);
    }
    ModelConfig {
        gbs: gbs,
        filename: ["./profiles/", "resnet50", "/graph.txt"].join(""),
        optimizer_memory_scaling: 2,
        pbs: 32,
        mbs: 32,
        papb: 70000000.0,
    }
}

// fn get_amoebanet36_model_config() -> ModelConfig {
//     let mut gbs = vec![32, 64];
//     for i in 1..((4096 - 64) / 64) + 1 {
//         gbs.push(64 + i * 64);
//     }
//     ModelConfig {
//         gbs: gbs,
//         filename: ["./profiles/", "vgg19", "/graph.txt"].join(""),
//         optimizer_memory_scaling: 2,
//         pbs: 32,
//         mbs: 32,
//         papb: 70000000.0,
//     }
// }

fn get_gnmt32_model_config() -> ModelConfig {
    let mut gbs = vec![64];
    for i in 1..((4096 - 64) / 64) + 1 {
        gbs.push(64 + i * 64);
    }
    ModelConfig {
        gbs: gbs,
        filename: ["./profiles/", "gnmt_32_simplified", "/graph.txt"].join(""),
        optimizer_memory_scaling: 3,
        pbs: 64,
        mbs: 64,
        papb: 1614725120.0 / 64.0,
    }
}

fn get_gnmt16_model_config() -> ModelConfig {
    let mut gbs = vec![64];
    for i in 1..((4096 - 64) / 64) + 1 {
        gbs.push(64 + i * 64);
    }
    ModelConfig {
        gbs: gbs,
        filename: ["./profiles/", "gnmt_large", "/graph.txt"].join(""),
        optimizer_memory_scaling: 3,
        pbs: 64,
        mbs: 64,
        papb: 100000000.0,
    }
}

fn get_bert48_model_config() -> ModelConfig {
    let mut gbs = vec![1, 2, 4, 8, 16];
    for i in 1..((256 - 16) / 8) + 1 {
        gbs.push(16 + i * 8);
    }
    ModelConfig {
        gbs: gbs,
        filename: ["./profiles/", "bert_48", "/graph.txt"].join(""),
        optimizer_memory_scaling: 3,
        pbs: 2,
        mbs: 1,
        papb: 1736689664.0 * 2.0,
    }
}
