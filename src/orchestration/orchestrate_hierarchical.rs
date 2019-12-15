use environment::device;
use model::model;
use orchestration::orchestrate::{Context, MatrixCell};
use orchestration::Conductor;
use rayon::prelude::*;
use std::collections::BTreeSet;

pub type Matrix = Vec<Vec<Vec<MatrixCell>>>;

#[derive(Debug)]
pub struct HierarchicalConductor<'a> {
    pub ctx: Context,
    pub m: model::Model,
    pub d: device::Devices,
    pub A: &'a mut Matrix,
}

impl<'a> HierarchicalConductor<'a> {
    pub fn compute_plan_hierarchical(
        &mut self,
        num_machines: u32,
        num_cards_per_machine: u32,
        bandwidth: f64,
        final_level: bool,
    ) {
        let compute_times = &self.m.perf.compute_times;
        let activation_sizes = &self.m.perf.activation_sizes;
        let output_activation_sizes = &self.m.perf.output_activation_sizes;
        let parameter_sizes = &self.m.perf.parameter_sizes;
        let A = &mut self.A; // pass mut ref for shorthand

        for _ in 0..compute_times.len() {
            let mut row_a: Vec<Vec<MatrixCell>> = vec![];
            for _ in 0..compute_times[0].len() {
                let mut row_row_a: Vec<MatrixCell> = vec![];
                for _ in 0..num_machines {
                    row_row_a.push(MatrixCell {
                        current_value: None,
                        current_maxmin_block: None,
                        optimal_split: None,
                        num_gpus_used: None,
                        availability_bitset: vec![],
                        gpu_ids: BTreeSet::new(),
                    })
                }
                row_a.push(row_row_a);
            }
            A.push(row_a);
        }

        for i in 0..compute_times.len() as usize {
            for j in i..compute_times[0].len() as usize {
                let cur_compute_time = compute_times[i][j];
                let cur_activation_size = activation_sizes[i][j];
                let cur_parameter_size = parameter_sizes[i][j];
                let max_m = num_machines;
                for m in 0..max_m as usize {
                    let mut dp_comm_time =
                        (4.0 * m as f64 * cur_parameter_size) / (bandwidth * (m + 1) as f64);
                    dp_comm_time /= num_cards_per_machine as f64;

                    if cur_compute_time > -0.5 {
                        A[i][j][m].current_value = Some((cur_compute_time + dp_comm_time) / (m + 1) as f64);
                        A[i][j][m].current_maxmin_block =
                            Some((cur_compute_time + dp_comm_time) / (m + 1) as f64);
                        A[i][j][m].optimal_split = None;
                        A[i][j][m].num_gpus_used = Some((m + 1) as u32);
                    }
                }
            }
        }

        let min_m = 1;
        // TODO: finish the rest

        //unimplemented!()
    }
}

impl<'a> Conductor for HierarchicalConductor<'a> {
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
