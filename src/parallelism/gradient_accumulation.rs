use analysis::*;
use environment::*;
use model::*;
use parallelism::data_parallel;
use std::collections::BTreeSet;

pub fn dp_ga_speedup(d: &device::Devices, m: &model::Model) -> f64 {
    data_parallel::dp_speedup(d, m)
}

/// get the current GA iteration batch size per machine
pub fn current_ga_iter_size(d: &device::Devices, m: &model::Model) -> u32 {
    let max_bs = gpu_memory::max_single_gpu_batch_size(m);
    let gbs = m.global_batch_size;

    if max_bs >= gbs {
        gbs
    } else {
        let mut iter = gbs / max_bs;
        if iter * max_bs < gbs {
            iter += 1
        }
        while iter < gbs / 2 + 1 {
            if gbs % iter != 0 {
                iter += 1;
            } else {
                break;
            }
        }
        gbs / iter
    }
}

pub fn optimal_ga_iter_size(d: &device::Devices, m: &model::Model) -> u32 {
    let max_bs = gpu_memory::max_single_gpu_batch_size(m);
    let gbs = m.global_batch_size;
    if max_bs >= gbs {
        gbs
    } else if max_bs * 2 >= gbs {
        gbs / 2
    } else {
        max_bs
    }
}

pub fn dp_cur_ga_p3_speedup(d: &device::Devices, m: &model::Model) -> f64 {
    let comp_time = m.perf.compute_times[0][m.perf.compute_times[0].len() - 1];
    let gbs = m.global_batch_size;
    let cur_max_bs = current_ga_iter_size(d, m);
    let partial: f64 = cur_max_bs as f64 / gbs as f64;

    let p3stats = cc_overlap::p3_partial(d, m, partial);
    comp_time / (comp_time / d.num_gpus as f64 + p3stats.offset)
}

pub fn dp_opt_ga_p3_speedup(d: &device::Devices, m: &model::Model) -> f64 {
    let comp_time = m.perf.compute_times[0][m.perf.compute_times[0].len() - 1];
    let gbs = m.global_batch_size;
    let opt_max_bs = optimal_ga_iter_size(d, m);
    let partial: f64 = opt_max_bs as f64 / gbs as f64;

    let p3stats = cc_overlap::p3_partial(d, m, partial);
    comp_time / (comp_time / d.num_gpus as f64 + p3stats.offset)
}
