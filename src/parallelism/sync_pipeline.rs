use environment::*;
use model::*;
use parallelism::*;
use std::collections::BTreeSet;

const VERBOSE: bool = false;

pub fn sync_pipeline_speedup_intermediate() {
    unimplemented!()
}

pub fn sync_pipeline_speedup(
    d: &device::Devices,
    m: &model::Model,
    rp: u32,
    pipeline_time: f64,
    p: Vec<(u32, u32, u32, BTreeSet<u32>)>,
) -> f64 {
    if VERBOSE {
        println!("[sync_pipeline] analysing plan:\n{:?}", p);
    }

    let compute_times = &m.perf.compute_times;
    let activation_sizes = &m.perf.activation_sizes;
    let parameter_sizes = &m.perf.parameter_sizes;
    let output_activation_sizes = &m.perf.output_activation_sizes;

    let total_compute_time = compute_times[0][compute_times[0].len() - 1];
    let m_batch = m.global_batch_size / rp / m.min_micro_batch_size;
    if m_batch == 0 {
        return 0.0;
    }
    if VERBOSE {
        println!("[sync_pipeline] using m_batch = {}", m_batch);
    }

    let block_time = pipeline_time / m_batch as f64;
    let pipeline_length_wout_dp =
        block_time * (m_batch - 1) as f64 + total_compute_time / rp as f64 / m_batch as f64;

    if VERBOSE {
        println!(
            "[sync_pipeline] block_time = {} | total/rp/m_batch = {}",
            block_time,
            total_compute_time / rp as f64 / m_batch as f64
        );
        println!(
            "[sync_pipeline] pipeline length without DP: {}",
            pipeline_length_wout_dp
        );
    }

    let mut pipeline_length_with_activations = pipeline_length_wout_dp;
    for i in 0..p.len() - 1 {
        let cut_activations =
            output_activation_sizes[(p[i].1 - 1) as usize] / rp as f64 / m_batch as f64;
        if VERBOSE {
            println!(
                "[sync_pipeline] cut_activations for stage {} ~ {} = {}, with original value = {}",
                i,
                i + 1,
                cut_activations,
                output_activation_sizes[(p[i].1 - 1) as usize]
            );
            println!(
                "[sync_pipeline] time needed for transmission = {}",
                split_concat::split_concat_all2all_time(d, &p[i].3, &p[i + 1].3, cut_activations)
            );
        }
        pipeline_length_with_activations +=
            split_concat::split_concat_all2all_time(d, &p[i].3, &p[i + 1].3, cut_activations);
    }

    if VERBOSE {
        println!(
            "[sync_pipeline] pipeline length after activations = {}",
            pipeline_length_with_activations
        );
    }

    let mut delta = 0.0;
    for i in 0..p.len() {
        let ARTime = data_parallel::all_reduce_time(
            d,
            &p[i].3,
            parameter_sizes[p[i].0 as usize][p[i].1 as usize],
        );
        if ARTime > i as f64 * block_time {
            delta = f64::max(ARTime - i as f64 * block_time, delta);
        }
    }

    if VERBOSE {
        println!(
            "[sync_pipeline] pipeline length after DP = {}",
            pipeline_length_with_activations + delta
        );
    }

    let res_speedup = total_compute_time / (pipeline_length_with_activations + delta);
    if VERBOSE {
        println!("Estimated Speedup: {}", res_speedup);
    }

    return res_speedup;
}
