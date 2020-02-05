use analysis::*;
use environment::*;
use model::*;
use parallelism::data_parallel;
use std::collections::BTreeSet;

const VERBOSE: bool = true;

pub fn dp_ga_speedup(d: &device::Devices, m: &model::Model) -> f64 {
    data_parallel::dp_speedup(d, m)
}

/// get the current GA iteration batch size per machine
pub fn current_ga_iter_size(d: &device::Devices, m: &model::Model) -> u32 {
    let max_bs = gpu_memory::max_single_gpu_batch_size(m);
    let bs_per_device = m.global_batch_size / d.num_gpus; // TODO: fix division info loss

    if max_bs >= bs_per_device {
        bs_per_device
    } else {
        let mut iter = bs_per_device / max_bs;
        if iter * max_bs < bs_per_device {
            iter += 1
        }
        while iter < bs_per_device / 2 + 1 {
            if bs_per_device % iter != 0 {
                iter += 1;
            } else {
                break;
            }
        }
        bs_per_device / iter
    }
}

pub fn optimal_ga_iter_size(d: &device::Devices, m: &model::Model) -> u32 {
    let max_bs = gpu_memory::max_single_gpu_batch_size(m);
    let bs_per_device = m.global_batch_size / d.num_gpus; // TODO: fix division info loss
    if max_bs >= bs_per_device {
        bs_per_device
    } else if max_bs * 2 >= bs_per_device {
        bs_per_device / 2
    } else {
        max_bs
    }
}

pub fn dp_cur_ga_p3_speedup(d: &device::Devices, m: &model::Model) -> f64 {
    dp_ga_overlap_speedup(d, m, current_ga_iter_size(d, m), true)
}

pub fn dp_opt_ga_p3_speedup(d: &device::Devices, m: &model::Model) -> f64 {
    dp_ga_overlap_speedup(d, m, optimal_ga_iter_size(d, m), true)
}

pub fn dp_cur_ga_inner_overlap_speedup(d: &device::Devices, m: &model::Model) -> f64 {
    dp_ga_overlap_speedup(d, m, current_ga_iter_size(d, m), false)
}

pub fn dp_ga_overlap_speedup(
    d: &device::Devices,
    m: &model::Model,
    ga_size: u32,
    inter_batch_overlap: bool,
) -> f64 {
    let comp_time = m.perf.compute_times[0][m.perf.compute_times[0].len() - 1];
    let gbs = m.global_batch_size;
    let partial: f64 = ga_size as f64 / gbs as f64;

    let overlap_stats = cc_overlap::cc_overlap_partial(d, m, inter_batch_overlap, partial);

    if VERBOSE {
        println!("[ga]\t Using GA Size: {}", ga_size);
        println!("[ga]\t Final Offset: {}", overlap_stats.offset)
    }
    comp_time / (comp_time / d.num_gpus as f64 + overlap_stats.offset)
}
