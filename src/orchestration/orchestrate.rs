use std::collections::BTreeSet;
use model::model;
use environment::device;

#[derive(Debug)]
struct matrix_cell {
    current_maxmin_block: f64,
    optimal_split: (u32, u32),
    num_gpus_used: u32,
    availability_bitset: Vec<bool>,
    gpu_ids: BTreeSet<u32>,
}

type Matrix = Vec<Vec<matrix_cell>>;

#[derive(Debug)]
pub struct Context {
    matrix: Matrix,
}

#[derive(Debug)]
pub struct Conductor <'a> {
    ctx: Context,
    m: model::Model,
    d: device::Devices,
    A: &'a mut Matrix,
}

impl<'a> Conductor<'a> {
    pub fn orchestrate_hierarchical() {
        unimplemented!()
    }
    pub fn orchestrate() {
        unimplemented!()
    }
}