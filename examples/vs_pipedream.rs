#![allow(non_snake_case)]

use ordered_float::OrderedFloat;
use rayon::prelude::*;

use std::collections::BTreeSet;

use HPGO::environment::*;
use HPGO::input::*;
use HPGO::layerwise::model::*;
use HPGO::layerwise::orchestration::*;
use HPGO::layerwise::parallelism::*;

const VERBOSE: bool = true;

struct ModelConfig {
    name: String,
    gbs: u32,
    // GBS vector
    filename: String,
    // filename to TorchGraph txt
    optimizer_memory_scaling: u32,
    pbs: u32,
    mbs: u32,
    papb: f64,
    partition: Vec<(u32, u32, u32, BTreeSet<u32>)>,
    pipedream: Vec<(u32, u32, u32, BTreeSet<u32>)>,
}

fn test_speedup_at_all_bs(mc: ModelConfig, flat: bool) -> (String, u32, f64, f64, f64, f64, f64) {
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
    let tgi: torch_graph::TorchGraphImporter = LayerwiseModelImporter::new();
    let result = tgi.ImportFrom(&mc.filename);
    let (perf, states) = (result.0.unwrap(), result.1.unwrap());
    if VERBOSE {
        println!("[main]\t Constructing HPGO Model...")
    }
    let mut m = model::Model::new_from_model_perf(perf, states, mc.pbs, mc.gbs);
    m.optimizer_memory_scaling = mc.optimizer_memory_scaling;
    m.min_micro_batch_size = mc.mbs;
    if mc.papb > 0.0 {
        m.peak_activation_per_batch = mc.papb;
    }

    if VERBOSE {
        println!("[main]\t Model Import Complete. Starting Parallel Planning...")
    }

    // Compute Max Batch Size in Parallel
    let gbs = mc.gbs;
    if VERBOSE {
        println!("[main]\t Planning in parallel for bs = {} ...", gbs);
    }
    // DP Speedups
    let dp_speedup = data_parallel::dp_speedup(&d, &m);
    // let dp_p3_speedup = data_parallel::dp_p3_speedup(&d16, &model);
    let dp_ga_p3_speedup = gradient_accumulation::dp_cur_ga_p3_speedup(&d, &m);
    let dp_ga_inner_overlap_speedup =
        gradient_accumulation::dp_cur_ga_inner_overlap_speedup(&d, &m);

    let hp_speedup = sync_pipeline::sync_pipeline_speedup_recursive(&d, &m, 1, mc.partition);
    let pd_speedup = sync_pipeline::sync_pipeline_speedup_recursive(&d, &m, 1, mc.pipedream);

    if VERBOSE {
        println!("[main]\t Got all results for bs = {} ...", gbs);
    }

    // return gbs and all speedups
    (
        mc.name,
        gbs,
        dp_speedup,
        dp_ga_p3_speedup,
        dp_ga_inner_overlap_speedup,
        hp_speedup,
        pd_speedup,
    )
}

fn main() {
    let mut results: Vec<(String, u32, f64, f64, f64, f64, f64)> = vec![];
    for x in vec![
        get_vgg19_model_config(),
        get_bertlarge_model_config(),
        get_xlnet_model_config(),
        get_amoebanet36_model_config(),
    ] {
        results.push(test_speedup_at_all_bs(x, false));
    }
    for x in results {
        println!("{:?}", x);
    }
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
    ModelConfig {
        name: "VGG-19".to_string(),
        gbs: 1024,
        filename: ["./profiles/", "vgg19", "/graph.txt"].join(""),
        optimizer_memory_scaling: 2,
        pbs: 32,
        mbs: 32,
        papb: 70000000.0,
        partition: vec![
            (0, 16, 14, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13].iter().cloned().collect()),
            (16, 24, 2, [14, 15].iter().cloned().collect()),
        ],
        pipedream: vec![
            (0, 11, 8, [0, 1, 2, 3, 4, 5, 6, 7].iter().cloned().collect()),
            (11, 17, 6, [8, 9, 10, 11, 12, 13].iter().cloned().collect()),
            (11, 19, 1, [14].iter().cloned().collect()),
            (19, 24, 1, [15].iter().cloned().collect()),
        ],
    }
}

fn get_bertlarge_model_config() -> ModelConfig {
    ModelConfig {
        name: "BERT Large".to_string(),
        gbs: 128,
        filename: ["./profiles/", "bert_large", "/graph.txt"].join(""),
        optimizer_memory_scaling: 3,
        pbs: 2,
        mbs: 1,
        papb: 1736689664.0,
        partition: vec![
            (0, 13, 8, [0, 1, 2, 3, 4, 5, 6, 7].iter().cloned().collect()),
            (13, 25, 8, [8, 9, 10, 11, 12, 13, 14, 15].iter().cloned().collect()),
        ],
        pipedream: vec![
            (0, 4, 2, [0, 1].iter().cloned().collect()),
            (4, 13, 6, [2,3,4,5,6,7].iter().cloned().collect()),
            (13, 16, 2, [8, 9].iter().cloned().collect()),
            (16, 19, 2, [10, 11].iter().cloned().collect()),
            (19, 22, 2, [12, 13].iter().cloned().collect()),
            (22, 25, 2, [14, 15].iter().cloned().collect()),
        ],
    }
}

fn get_xlnet_model_config() -> ModelConfig {
    ModelConfig {
        name: "XLNet 36".to_string(),
        gbs: 128,
        filename: ["./profiles/", "xlnet_36", "/graph.txt"].join(""),
        optimizer_memory_scaling: 3,
        pbs: 1,
        mbs: 1,
        papb: 3942774528.0 * 1.5,
        partition: vec![
            (0, 22, 8, [0, 1, 2, 3, 4, 5, 6, 7].iter().cloned().collect()),
            (22, 40, 8, [8, 9, 10, 11, 12, 13, 14, 15].iter().cloned().collect()),
        ],
        pipedream: vec![
            (0, 6, 1, [0].iter().cloned().collect()),
            (6, 8, 1, [1].iter().cloned().collect()),
            (8, 11, 1, [2].iter().cloned().collect()),
            (11, 14, 1, [3].iter().cloned().collect()),
            (14, 17, 1, [4].iter().cloned().collect()),
            (17, 20, 1, [5].iter().cloned().collect()),
            (20, 23, 1, [6].iter().cloned().collect()),
            (23, 26, 1, [7].iter().cloned().collect()),
            (26, 29, 2, [8,9].iter().cloned().collect()),
            (29, 31, 1, [10].iter().cloned().collect()),
            (31, 33, 1, [11].iter().cloned().collect()),
            (33, 35, 1, [12].iter().cloned().collect()),
            (35, 37, 1, [13].iter().cloned().collect()),
            (37, 40, 2, [14, 15].iter().cloned().collect()),
        ],
    }
}

fn get_amoebanet36_model_config() -> ModelConfig {
    ModelConfig {
        name: "AmoebaNet-D 36".to_string(),
        gbs: 128,
        filename: ["./profiles/", "amoebanet_36", "/graph.txt"].join(""),
        optimizer_memory_scaling: 2,
        pbs: 1,
        mbs: 1,
        papb: 250845152.0 * 1.5,
        partition: vec![
            (0, 30, 8, [0, 1, 2, 3, 4, 5, 6, 7].iter().cloned().collect()),
            (30, 42, 8, [8, 9, 10, 11, 12, 13, 14, 15].iter().cloned().collect()),
        ],
        pipedream: vec![
            (0, 5, 1, [0].iter().cloned().collect()),
            (5, 8, 1, [1].iter().cloned().collect()),
            (8, 11, 1, [2].iter().cloned().collect()),
            (11, 14, 1, [3].iter().cloned().collect()),
            (14, 17, 1, [4].iter().cloned().collect()),
            (17, 19, 1, [5].iter().cloned().collect()),
            (19, 22, 1, [6].iter().cloned().collect()),
            (22, 25, 1, [7].iter().cloned().collect()),
            (25, 27, 1, [8].iter().cloned().collect()),
            (27, 29, 1, [9].iter().cloned().collect()),
            (29, 31, 1, [10].iter().cloned().collect()),
            (31, 33, 1, [11].iter().cloned().collect()),
            (33, 35, 1, [12].iter().cloned().collect()),
            (35, 37, 1, [13].iter().cloned().collect()),
            (37, 39, 1, [14].iter().cloned().collect()),
            (39, 42, 1, [15].iter().cloned().collect()),
        ],
    }
}
