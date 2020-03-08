use crate::environment::*;
use crate::layerwise::model::*;

use std::collections::BTreeSet;

/// Calculate the speedup for the partition, assuming GPipe microbatch arrangement
pub fn gpipe_pipeline_speedup(
    _d: &device::Devices,
    _m: &model::Model,
    _rp: u32,
    _pipeline_time: f64,
    _p: Vec<(u32, u32, u32, BTreeSet<u32>)>,
) -> f64 {
    unimplemented!()
}
