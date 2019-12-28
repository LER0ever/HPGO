use environment::*;
use model::*;
use std::collections::BTreeSet;

pub fn all_reduce_time(d: &device::Devices, gids: &BTreeSet<u32>, size: f64) -> f64 {
    let b_cross = d.is_cross_machine_within(gids);
    let f_factor: f64 = (gids.len() - 1) as f64 / gids.len() as f64;
    match gids.len() {
        1 => 0.0,
        _ => match b_cross {
            true => size * 2.0 * f_factor / ethernet::BANDWIDTH_ETHERNET_NCCL,
            false => size * 2.0 * f_factor / nvlink::BANDWIDTH_NVLINK_P2P / (gids.len() / 2) as f64,
        },
    }
}

pub fn dp_speedup_strong(d: &device::Devices, compute: f64, all_reduce: f64) -> f64 {
    compute / (compute / d.num_gpus as f64 + all_reduce)
}

pub fn dp_speedup_weak(d: &device::Devices, compute: f64, all_reduce: f64) -> f64 {
    d.num_gpus as f64 * (compute / (compute + all_reduce))
}
