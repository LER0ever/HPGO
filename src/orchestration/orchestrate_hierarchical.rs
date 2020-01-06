use environment::device;
use input::*;
use model::model;
use orchestration::orchestrate_async::MatrixCell;
use orchestration::{Orchestrate, OrchestrationResult};
use rayon::prelude::*;
use std::collections::BTreeSet;

pub type Matrix = Vec<Vec<Vec<MatrixCell>>>;

#[derive(Debug)]
pub struct HierarchicalConductor {
    pub ctx: Matrix,
    pub m: model::Model,
    // pub d: device::Devices,
    //pub A: &'a mut Matrix,
}

impl HierarchicalConductor {
    pub fn new_from_torch_graph(filename: &str, pbs: u32, gbs: u32) -> HierarchicalConductor {
        let tgi: torch_graph::TorchGraphImporter = ModelImporter::new();
        let result = tgi.ImportFrom(filename);
        let (perf, states) = (result.0.unwrap(), result.1.unwrap());
        let model = model::Model::new_from_model_perf(perf, states, pbs, gbs);
        HierarchicalConductor {
            ctx: vec![],
            m: model,
            //A: &mut vec![],
        }
    }
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
        let all_predecessor_ids = &self.m.perf.all_predecessor_ids;
        let A = &mut self.ctx; // pass mut ref for shorthand

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
                    });
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
                        A[i][j][m].current_value =
                            Some((cur_compute_time + dp_comm_time) / (m + 1) as f64);
                        A[i][j][m].current_maxmin_block =
                            Some((cur_compute_time + dp_comm_time) / (m + 1) as f64);
                        A[i][j][m].optimal_split = None;
                        A[i][j][m].num_gpus_used = Some((m + 1) as u32);
                    }
                }
            }
        }

        let min_m = 1;
        let max_i = if final_level {
            1
        } else {
            compute_times.len() as u32
        };

        for i in 0..max_i as usize {
            for m in min_m..num_machines as usize {
                for j in i + 1..compute_times[0].len() {
                    let (mut min_pipeline_time, mut optimal_split, mut optimal_num_machines) = (
                        A[i][j][m].current_maxmin_block,
                        A[i][j][m].optimal_split,
                        A[i][j][m].num_gpus_used,
                    );
                    for k in all_predecessor_ids[j].iter() {
                        if i > 0 && all_predecessor_ids[i - 1].contains(k) {
                            continue;
                        }
                        let max_mp = m + 1;
                        for mp in 1..max_mp {
                            let input_transfer_time = (2.0 * output_activation_sizes[*k as usize])
                                / (bandwidth * mp as f64);
                            // TODO: output_transfer_time
                            let mut last_stage_time = compute_times[(*k + 1) as usize][j];
                            if last_stage_time < -0.5 {
                                continue;
                            }
                            let last_stage_parameter_size = parameter_sizes[(*k + 1) as usize][j];
                            last_stage_time += (4.0 * (mp - 1) as f64 * last_stage_parameter_size)
                                / (bandwidth * mp as f64);
                            last_stage_time /= mp as f64;

                            if A[i][*k as usize][m - mp].current_maxmin_block == None
                                || A[i][*k as usize][m - mp].current_maxmin_block.unwrap() < -0.5
                            {
                                continue;
                            }

                            let mut pipeline_time = f64::max(
                                A[i][*k as usize][m - mp].current_maxmin_block.unwrap(),
                                last_stage_time,
                            );
                            pipeline_time = f64::max(pipeline_time, input_transfer_time);

                            if min_pipeline_time.is_none()
                                || (min_pipeline_time.is_some()
                                    && pipeline_time < min_pipeline_time.unwrap())
                            {
                                optimal_split = Some((*k, m as u32 - mp as u32));
                                optimal_num_machines = Some(mp as u32);
                                min_pipeline_time = Some(pipeline_time);
                            }
                        }
                    }
                    A[i][j][m].current_maxmin_block = min_pipeline_time;
                    A[i][j][m].optimal_split = optimal_split;
                    A[i][j][m].num_gpus_used = optimal_num_machines;
                }
            }
        }

        // return A;
    }
}

impl Orchestrate for HierarchicalConductor {
    fn orchestrate(&mut self) {
        unimplemented!()
    }

    fn compute_plan(&mut self) {
        unimplemented!()
    }

    fn analyse_plan(&self) {
        unimplemented!()
    }

    fn return_plan(&self) -> Box<dyn OrchestrationResult> {
        unimplemented!()
    }
}
