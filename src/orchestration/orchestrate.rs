use environment::device;
use model::model;
use orchestration::Conductor;
use std::collections::BTreeSet;

#[derive(Debug)]
pub struct MatrixCell {
    pub current_value: Option<f64>,
    pub current_maxmin_block: Option<f64>,
    pub optimal_split: Option<(u32, u32)>,
    pub num_gpus_used: Option<u32>,
    pub availability_bitset: Vec<bool>,
    pub gpu_ids: BTreeSet<u32>,
}

pub type Matrix = Vec<Vec<MatrixCell>>;

#[derive(Debug)]
pub struct Context {
    matrix: Matrix,
}

#[derive(Debug)]
pub struct SyncConductor<'a> {
    pub ctx: Context,
    pub m: model::Model,
    pub d: device::Devices,
    pub A: &'a mut Matrix,
}

impl<'a> Conductor for SyncConductor<'a> {
    fn orchestrate(&self) {
        unimplemented!()
    }

    fn compute_plan(&mut self) {
        unimplemented!()
    }

    fn analyse_plan(&self) {
        unimplemented!()
    }
}
