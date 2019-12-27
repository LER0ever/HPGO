use environment::device;
use input::*;
use model::model;
use orchestration::{Conductor, OrchestrationResult};
use parallelism::*;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

pub type bitset = Vec<bool>;

#[derive(Debug)]
pub struct MatrixCell {
    pub current_value: Option<f64>,
    pub current_maxmin_block: Option<f64>,
    pub optimal_split: Option<(u32, u32)>,
    pub num_gpus_used: Option<u32>,
    pub availability_bitset: bitset,
    pub gpu_ids: BTreeSet<u32>,
}

pub type Matrix = Vec<Vec<RefCell<BTreeMap<bitset, MatrixCell>>>>;

// #[derive(Debug)]
// pub struct Context {
//     pub matrix: Matrix,
// }

pub struct SyncConductorResult {
    speedup_no_overlap: f64,
    speedup_p3: f64,
    splits: Vec<u32>,
    stages: Vec<(u32, u32, u32, BTreeSet<u32>)>,
}

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

    pub fn compute_plan_sync(&mut self, spa_size: u32, rp: u32) {
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
                    println!(
                        "[orchestrate] checking DP allreduce time for {:?}: {}",
                        &n.gids, data_parallel::all_reduce_time(d, &n.gids, cur_parameter_size)
                    );
                    A[j][m as usize].get_mut().insert(
                        ph.clone(),
                        MatrixCell {
                            current_value: None,
                            current_maxmin_block: Some(
                                f64::max((cur_compute_time) / (m + 1) as f64 / rp as f64, 0.0)
                                    + data_parallel::all_reduce_time(
                                        d,
                                        &n.gids,
                                        cur_parameter_size
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

                let mut cur_A_bt = A[j][m as usize].borrow_mut();
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
                    cur_A.availability_bitset.clone(),
                    cur_A.gpu_ids.clone(),
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
                                    || A[*k as usize][(m - mp) as usize]
                                        .borrow()
                                        .get(&ph)
                                        .unwrap()
                                        .current_maxmin_block
                                        .unwrap()
                                        < -0.5
                                {
                                    continue;
                                }

                                let mut pipeline_time = f64::max(
                                    A[*k as usize][(m - mp) as usize]
                                        .borrow()
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
                                    last_from = bs.clone();
                                    last_machines = nbs.gids.clone();
                                }

                                if !cur_A_bt.contains_key(&nbs.occupied)
                                    || pipeline_time
                                        < cur_A_bt
                                            .get(&nbs.occupied)
                                            .unwrap()
                                            .current_maxmin_block
                                            .unwrap()
                                {
                                    println!(
                                        "Updating A[{}][{}][{:?}] \t| maxmin_block: {:.7}\t split: {:?}\t from_bs: {:?}\t gids: {:?} ",
                                        j,
                                        m,
                                        &nbs.occupied.iter().fold(String::new(), |acc, &b| acc
                                            + &(b as i32).to_string()),
                                        pipeline_time,
                                        (*k, m - mp),
                                        &bs.iter().fold(String::new(), |acc, &b| acc
                                            + &(b as i32).to_string()),
                                        nbs.gids.clone(),
                                    );
                                    cur_A_bt.insert(
                                        nbs.occupied.clone(),
                                        MatrixCell {
                                            current_value: None,
                                            current_maxmin_block: Some(pipeline_time),
                                            optimal_split: Some((*k, m - mp)),
                                            num_gpus_used: Some(mp),
                                            availability_bitset: bs.clone(),
                                            gpu_ids: nbs.gids.clone(),
                                        },
                                    );
                                }
                            }
                        }
                    }
                }

                cur_A_bt.insert(
                    ph.clone(),
                    MatrixCell {
                        current_value: None,
                        current_maxmin_block: min_pipeline_time,
                        optimal_split: optimal_split,
                        num_gpus_used: optimal_num_machines,
                        availability_bitset: last_from.clone(),
                        gpu_ids: last_machines.clone(),
                    },
                );
            }
        }
    }

    pub fn analyse_plan_sync(
        &self,
        end: u32,
        num_machines: u32,
        rp: u32,
    ) -> Vec<(u32, u32, u32, BTreeSet<u32>)> {
        let mut res: Vec<(u32, u32, u32, BTreeSet<u32>)> = vec![];
        let mut ph: bitset = vec![];
        for _ in 0..num_machines * rp + 1 {
            ph.push(true);
        }
        let mut mt = self.A[end as usize - 1][num_machines as usize - 1].borrow();
        let mut metadata = mt.get(&ph).unwrap();

        let mut next_split = metadata.optimal_split;
        let mut last_machines = metadata.gpu_ids.clone();
        let mut last_from = metadata.availability_bitset.clone();
        let mut prev_split = end - 1;

        while !next_split.is_none() {
            let num_machines_used = metadata.num_gpus_used.unwrap();
            res.push((
                next_split.unwrap().0 + 1,
                prev_split,
                num_machines_used,
                last_machines,
            ));
            prev_split = res[res.len() - 1].0;

            mt = self.A[next_split.unwrap().0 as usize][next_split.unwrap().1 as usize].borrow();
            metadata = mt.get(&last_from).unwrap();
            next_split = metadata.optimal_split;
            last_machines = metadata.gpu_ids.clone();
            last_from = metadata.availability_bitset.clone();
        }

        let num_machines_used = metadata.num_gpus_used.unwrap();
        res.push((0, prev_split, num_machines_used, last_machines));
        res.reverse();

        res
    }
}

impl OrchestrationResult for SyncConductorResult {
    fn get_speedup(&self) -> Option<f64> {
        unimplemented!()
    }
    fn get_splits(&self) -> Option<Vec<u32>> {
        unimplemented!()
    }

    fn pretty_print(&self) -> Option<String> {
        unimplemented!()
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

    fn return_plan(&self) -> Box<dyn OrchestrationResult> {
        unimplemented!()
    }
}