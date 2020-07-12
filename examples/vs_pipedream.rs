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
        gbs: 2048,
        filename: ["./profiles/", "vgg19", "/graph.txt"].join(""),
        optimizer_memory_scaling: 2,
        pbs: 32,
        mbs: 32,
        papb: 70000000.0,
        partition: vec![
            (0, 20, 31, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30].iter().cloned().collect()),
            (20, 24, 2, [31].iter().cloned().collect()),
        ],
        pipedream: vec![
            (0, 10, 6, [0, 1, 2, 3, 4, 5, 8, 9,10,11,12,13, 16,17,18,19,20,21].iter().cloned().collect()),
            (10, 16, 2, [6, 7, 14,15, 22,23].iter().cloned().collect()),
            (16, 21, 7, [24,25,26,27,28,29,30].iter().cloned().collect()),
            (21, 24, 1, [31].iter().cloned().collect()),
        ],
    }
}

fn get_bertlarge_model_config() -> ModelConfig {
    ModelConfig {
        name: "BERT Large".to_string(),
        gbs: 256,
        filename: ["./profiles/", "bert_large", "/graph.txt"].join(""),
        optimizer_memory_scaling: 3,
        pbs: 2,
        mbs: 1,
        papb: 1736689664.0,
        // partition: vec![
        //     (0, 7, 8, [0, 1, 2, 3, 4, 5, 6, 7].iter().cloned().collect()),
        //     (7, 13, 8, [8, 9, 10, 11, 12, 13, 14, 15].iter().cloned().collect()),
        //     (13, 19, 8, [16, 17, 18, 19, 20, 21, 22, 23].iter().cloned().collect()),
        //     (19, 25, 8, [24, 25, 26, 27, 28, 29, 30, 31].iter().cloned().collect()),
        // ],
        partition: vec![
            (0, 12, 16, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15].iter().cloned().collect()),
            (12, 25, 16, [16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31].iter().cloned().collect()),
        ],
        pipedream: vec![
            (0, 3, 3, [0, 1, 2].iter().cloned().collect()),
            (3, 7, 5, [3,4,5,6,7].iter().cloned().collect()),
            (7, 10, 4, [8, 9, 10, 11].iter().cloned().collect()),
            (10, 13, 4, [12, 13, 14, 15].iter().cloned().collect()),
            (13, 16, 4, [16, 17, 18, 19].iter().cloned().collect()),
            (16, 19, 4, [20, 21, 22, 23].iter().cloned().collect()),
            (19, 22, 4, [24, 25, 26, 27].iter().cloned().collect()),
            (22, 25, 4, [28, 29, 30, 31].iter().cloned().collect()),
        ],
    }
}

fn get_xlnet_model_config() -> ModelConfig {
    ModelConfig {
        name: "XLNet 36".to_string(),
        gbs: 256,
        filename: ["./profiles/", "xlnet_36", "/graph.txt"].join(""),
        optimizer_memory_scaling: 3,
        pbs: 1,
        mbs: 1,
        papb: 3942774528.0 * 1.5,
        partition: vec![
            (0, 22, 16, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15].iter().cloned().collect()),
            (22, 40, 16, [16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31].iter().cloned().collect()),
        ],
        // partition: vec![
        //     (0, 7, 4, [0,1,2,3].iter().cloned().collect()),
        //     (7, 13, 4, [4,5,6,7].iter().cloned().collect()),
        //     (13, 22, 8, [8,9,10,11,12,13,14,15].iter().cloned().collect()),
        //     (22, 40, 16, [16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31].iter().cloned().collect()),
        // ],
        pipedream: vec![
            (0, 5, 1, [0].iter().cloned().collect()),
            (5, 13, 7, [1,2,3,4,5,6,7].iter().cloned().collect()),
            (13, 22, 8, [8,9,10,11,12,13,14,15].iter().cloned().collect()),
            (22, 31, 8, [16,17,18,19,20,21,22,23].iter().cloned().collect()),
            (31, 39, 7, [24,25,26,27,28,29,30].iter().cloned().collect()),
            (39, 40, 1, [31].iter().cloned().collect()),
        ],
    }
}

fn get_amoebanet36_model_config() -> ModelConfig {
    ModelConfig {
        name: "AmoebaNet-D 36".to_string(),
        gbs: 256,
        filename: ["./profiles/", "amoebanet_36", "/graph.txt"].join(""),
        optimizer_memory_scaling: 2,
        pbs: 1,
        mbs: 1,
        papb: 250845152.0 * 1.5,
        partition: vec![
            (0, 30, 16, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15].iter().cloned().collect()),
            (30, 42, 16, [16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31].iter().cloned().collect()),
        ],
        // partition: vec![
        //     (0, 13, 8, [0, 1, 2, 3, 4, 5, 6, 7].iter().cloned().collect()),
        //     (13, 25, 8, [8, 9, 10, 11, 12, 13, 14, 15].iter().cloned().collect()),
        //     (25, 36, 8, [16, 17, 18, 19, 20, 21, 22, 23].iter().cloned().collect()),
        //     (36, 42, 8, [24, 25, 26, 27, 28, 29, 30, 31].iter().cloned().collect()),
        // ],
        // partition: vec![
        //     (0, 3, 1, [0].iter().cloned().collect()),
        //     (3, 5, 1, [1].iter().cloned().collect()),
        //     (5, 7, 1, [2].iter().cloned().collect()),
        //     (7, 9, 1, [3].iter().cloned().collect()),
        //     (9, 11, 1, [4].iter().cloned().collect()),
        //     (11, 13, 1, [5].iter().cloned().collect()),
        //     (13, 15, 1, [6].iter().cloned().collect()),
        //     (15, 17, 1, [7].iter().cloned().collect()),
        //     (17, 18, 1, [8].iter().cloned().collect()),
        //     (18, 19, 1, [9].iter().cloned().collect()),
        //     (19, 21, 1, [10].iter().cloned().collect()),
        //     (21, 22, 1, [11].iter().cloned().collect()),
        //     (22, 23, 1, [12].iter().cloned().collect()),
        //     (23, 24, 1, [13].iter().cloned().collect()),
        //     (24, 25, 1, [14].iter().cloned().collect()),
        //     (25, 26, 1, [15].iter().cloned().collect()),
        //     (26, 28, 1, [16].iter().cloned().collect()),
        //     (28, 29, 1, [17].iter().cloned().collect()),
        //     (29, 30, 1, [18].iter().cloned().collect()),
        //     (30, 31, 1, [19].iter().cloned().collect()),
        //     (31, 32, 1, [20].iter().cloned().collect()),
        //     (32, 33, 1, [21].iter().cloned().collect()),
        //     (33, 34, 1, [22].iter().cloned().collect()),
        //     (34, 35, 1, [23].iter().cloned().collect()),
        //     (35, 36, 1, [24].iter().cloned().collect()),
        //     (36, 37, 1, [25].iter().cloned().collect()),
        //     (37, 38, 1, [26].iter().cloned().collect()),
        //     (38, 39, 1, [27].iter().cloned().collect()),
        //     (39, 40, 1, [28].iter().cloned().collect()),
        //     (40, 41, 1, [29].iter().cloned().collect()),
        //     (41, 42, 1, [30].iter().cloned().collect()),
        //     (42, 42, 1, [31].iter().cloned().collect()),
        // ],
        pipedream: vec![
            (0, 7, 3, [0, 1, 2].iter().cloned().collect()),
            (7, 9, 1, [3].iter().cloned().collect()),
            (9, 11, 1, [4].iter().cloned().collect()),
            (11, 13, 1, [5].iter().cloned().collect()),
            (13, 15, 1, [6].iter().cloned().collect()),
            (15, 17, 1, [7].iter().cloned().collect()),
            (17, 18, 1, [8].iter().cloned().collect()),
            (18, 19, 1, [9].iter().cloned().collect()),
            (19, 21, 1, [10].iter().cloned().collect()),
            (21, 22, 1, [11].iter().cloned().collect()),
            (22, 24, 1, [12].iter().cloned().collect()),
            (24, 26, 1, [13].iter().cloned().collect()),
            (26, 28, 1, [14].iter().cloned().collect()),
            (28, 29, 1, [15].iter().cloned().collect()),
            (29, 30, 3, [16, 17, 18].iter().cloned().collect()),
            (30, 31, 1, [19].iter().cloned().collect()),
            (31, 32, 1, [20].iter().cloned().collect()),
            (32, 33, 1, [21].iter().cloned().collect()),
            (33, 34, 1, [22].iter().cloned().collect()),
            (34, 35, 1, [23].iter().cloned().collect()),
            (35, 36, 3, [24, 25, 26].iter().cloned().collect()),
            (36, 37, 1, [27].iter().cloned().collect()),
            (37, 38, 1, [28].iter().cloned().collect()),
            (38, 39, 1, [29].iter().cloned().collect()),
            (39, 40, 1, [30].iter().cloned().collect()),
            (40, 42, 1, [31].iter().cloned().collect()),
        ],

    }
}
