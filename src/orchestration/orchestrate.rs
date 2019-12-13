use environment::device;
use model::model;
use std::collections::BTreeSet;

#[derive(Debug)]
pub struct MatrixCell {
    current_maxmin_block: f64,
    optimal_split: (u32, u32),
    num_gpus_used: u32,
    availability_bitset: Vec<bool>,
    gpu_ids: BTreeSet<u32>,
}

pub type Matrix = Vec<Vec<MatrixCell>>;

#[derive(Debug)]
pub struct Context {
    matrix: Matrix,
}

#[derive(Debug)]
pub struct Conductor<'a> {
    pub ctx: Context,
    pub m: model::Model,
    pub d: device::Devices,
    pub A: &'a mut Matrix,
}

impl<'a> Conductor<'a> {
    pub fn orchestrate(&self) {
        unimplemented!()
    }
}
