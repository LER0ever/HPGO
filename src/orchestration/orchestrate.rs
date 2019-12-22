use environment::device;
use input::*;
use model::model;
use orchestration::Conductor;
use parallelism::*;
use std::collections::{BTreeMap, BTreeSet};
use std::cell::RefCell;

#[derive(Debug)]
pub struct MatrixCell {
    pub current_value: Option<f64>,
    pub current_maxmin_block: Option<f64>,
    pub optimal_split: Option<(u32, u32)>,
    pub num_gpus_used: Option<u32>,
    pub availability_bitset: Vec<bool>,
    pub gpu_ids: BTreeSet<u32>,
}

pub type bitset = Vec<bool>;
pub type Matrix = Vec<Vec<RefCell<BTreeMap<bitset, MatrixCell>>>>;

// #[derive(Debug)]
// pub struct Context {
//     pub matrix: Matrix,
// }

pub struct SyncConductor {
    pub A: Matrix,
    pub m: model::Model,
    pub d: device::Devices,
}

impl SyncConductor {
    pub fn new(filename: &str, seps: Vec<u32>) -> SyncConductor {
        let tgi: torch_graph::TorchGraphImporter = ModelImporter::new();
        let result = tgi.ImportFrom(filename);
        let (perf, states) = (result.0.unwrap(), result.1.unwrap());
        let model = model::Model::new_from_model_perf(perf, states);
        let n = seps[seps.len() - 1];
        let d = device::Devices::new(n, seps);
        SyncConductor {
            A: vec![],
            m: model,
            d: d,
        }
    }

    pub fn compute_plan(&mut self, spa_size: u32, rp: u32) {
        // Shorthands
        let compute_times = &self.m.perf.compute_times;
        let activation_sizes = &self.m.perf.activation_sizes;
        let output_activation_sizes = &self.m.perf.output_activation_sizes;
        let parameter_sizes = &self.m.perf.parameter_sizes;
        let all_predecessor_ids = &self.m.perf.all_predecessor_ids;
        let A = &mut self.A;
        let d = &self.d;
        let num_machines = spa_size;

        // Initialize ctx matrix
        for _ in 0..compute_times[0].len() {
            let mut row_a: Vec<RefCell<BTreeMap<bitset, MatrixCell>>> = vec![];
            for _ in 0..num_machines {
                // let mut bt = ;
                row_a.push(RefCell::new(BTreeMap::new()));
            }
            A.push(row_a);
        }

        // Bitset placeholder
        let mut ph: bitset = vec![];
        let mut empty: bitset = vec![];
        for _ in 0..num_machines * rp + 1 {
            ph.push(true);
        }
        for _ in 0..num_machines * rp {
            empty.push(false);
        }

        // DP Initialization
        for j in 0..compute_times[0].len() {
            let cur_compute_time = compute_times[0][j];
            let cur_activation_size = activation_sizes[0][j];
            let cur_parameter_size = parameter_sizes[0][j];
            let max_m = num_machines;
            for m in 0..max_m {
                let n = d.next_cards(empty.clone(), (m + 1) * rp)[0].clone();
                if cur_compute_time < -0.5 {
                    // A[j][m][ph] =
                    A[j][m as usize].get_mut().insert(
                        ph.clone(),
                        MatrixCell {
                            current_value: None,
                            current_maxmin_block: None,
                            optimal_split: None,
                            num_gpus_used: None,
                            availability_bitset: ph.clone(),
                            gpu_ids: BTreeSet::new(),
                        },
                    );
                    A[j][m as usize].get_mut().insert(
                        n.occupied.clone(),
                        MatrixCell {
                            current_value: None,
                            current_maxmin_block: None,
                            optimal_split: None,
                            num_gpus_used: None,
                            availability_bitset: ph.clone(),
                            gpu_ids: n.gids,
                        },
                    );
                } else {
                    A[j][m as usize].get_mut().insert(
                        ph.clone(),
                        MatrixCell {
                            current_value: None,
                            current_maxmin_block: Some(
                                f64::max((cur_compute_time) / (m + 1) as f64 / rp as f64, 0.0)
                                    + data_parallel::all_reduce_time(
                                        d,
                                        &n.gids,
                                        cur_parameter_size,
                                    ),
                            ),
                            optimal_split: None,
                            num_gpus_used: Some(m + 1),
                            availability_bitset: empty.clone(),
                            gpu_ids: BTreeSet::new(),
                        },
                    );
                    A[j][m as usize].get_mut().insert(
                        n.occupied.clone(),
                        MatrixCell {
                            current_value: None,
                            current_maxmin_block: Some(
                                f64::max((cur_compute_time) / (m + 1) as f64 / rp as f64, 0.0)
                                    + data_parallel::all_reduce_time(
                                        d,
                                        &n.gids,
                                        cur_parameter_size,
                                    ),
                            ),
                            optimal_split: None,
                            num_gpus_used: Some(m + 1),
                            availability_bitset: empty.clone(),
                            gpu_ids: n.gids,
                        },
                    );
                }
            }
        }

        let min_m = 1;
        for m in min_m..num_machines {
            for j in 1..compute_times[0].len() {
                if !A[j][m as usize].borrow().contains_key(&ph) {
                    continue;
                }

                println!("m = {}, j = {}", m, j);

                let cur_A_bt = A[j][m as usize].borrow();
                let cur_A = cur_A_bt.get(&ph).unwrap();
                let (
                    mut min_pipeline_time,
                    mut optimal_split,
                    mut optimal_num_machines,
                    mut last_from,
                    mut last_machines,
                ) = (
                    cur_A.current_maxmin_block,
                    cur_A.optimal_split,
                    cur_A.num_gpus_used,
                    &cur_A.availability_bitset,
                    &cur_A.gpu_ids,
                );

                for k in all_predecessor_ids[j].iter() {
                    let max_mp = m + 1;
                    for mp in 1..max_mp {
                        for (bs, cell) in A[*k as usize][(m - mp) as usize].borrow().iter() {
                            if bs.len() as u32 > num_machines * rp {
                                continue; // skip ph
                            }

                            let next_bs_all = d.next_cards_with_replica(bs.to_vec(), mp, rp);

                            for nbs in next_bs_all {
                                let from = &cell.gpu_ids;
                                let to = &nbs.gids;

                                let input_transfer_time = split_concat::split_concat_all2all_time(
                                    d,
                                    from,
                                    to,
                                    2.0 * output_activation_sizes[*k as usize],
                                );

                                let mut last_stage_time = compute_times[*k as usize + 1][j];
                                if last_stage_time < -0.5 {
                                    continue;
                                }
                                last_stage_time /= (mp * rp) as f64;

                                if !A[*k as usize][(m - mp) as usize].borrow().contains_key(&ph)
                                    || A[*k as usize][(m - mp) as usize].borrow()
                                        .get(&ph)
                                        .unwrap()
                                        .current_maxmin_block
                                        .unwrap()
                                        < -0.5
                                {
                                    continue;
                                }

                                let mut pipeline_time = f64::max(
                                    A[*k as usize][(m - mp) as usize].borrow()
                                        .get(bs)
                                        .unwrap()
                                        .current_maxmin_block
                                        .unwrap(),
                                    last_stage_time,
                                );
                                pipeline_time = f64::max(pipeline_time, input_transfer_time);

                                if min_pipeline_time.is_none()
                                    || pipeline_time < min_pipeline_time.unwrap()
                                {
                                    optimal_split = Some((*k, m - mp));
                                    optimal_num_machines = Some(mp);
                                    min_pipeline_time = Some(pipeline_time);
                                    last_from = &bs;
                                    last_machines = &nbs.gids;
                                }

                                if A[j][m as usize].borrow().contains_key(&nbs.occupied)
                                    || pipeline_time
                                        < A[j][m as usize].borrow()
                                            .get(&nbs.occupied)
                                            .unwrap()
                                            .current_maxmin_block
                                            .unwrap()
                                {
                                    A[j][m as usize].borrow_mut().insert(
                                        nbs.occupied,
                                        MatrixCell {
                                            current_value: None,
                                            current_maxmin_block: Some(pipeline_time),
                                            optimal_split: Some((*k, m-mp)),
                                            num_gpus_used: Some(mp),
                                            availability_bitset: bs.clone(),
                                            gpu_ids: nbs.gids,
                                        },
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Conductor for SyncConductor {
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
