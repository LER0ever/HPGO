use environment::device;
use input::*;
use itertools::sorted;
use model::model;
use orchestration::{Orchestrate, OrchestrationResult};
use parallelism::*;
use pyo3::prelude::*;
use rayon::prelude::*;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

const VERBOSE: bool = true;

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

/// Orchestration result returned by SyncConductor
#[pyclass]
#[derive(Debug, Clone)]
pub struct SyncOrchestrateResult {
    pub speedup: f64,
    pub stages: Vec<(u32, u32, u32, BTreeSet<u32>)>,
}

/// Conductor for Synchronous Pipeline
#[pyclass]
#[derive(Debug, Clone)]
pub struct SyncOrchestrate {
    #[pyo3(get)]
    pub m: model::Model,
    #[pyo3(get)]
    pub d: device::Devices,
    #[pyo3(get)]
    pub res: Vec<SyncOrchestrateResult>,
}

impl SyncOrchestrate {
    /// Construct the SyncConductor from TorchGraphImporter
    pub fn new_from_torch_graph(
        filename: &str,
        pbs: u32,
        gbs: u32,
        seps: Vec<u32>,
    ) -> SyncOrchestrate {
        let tgi: torch_graph::TorchGraphImporter = ModelImporter::new();
        let result = tgi.ImportFrom(filename);
        let (perf, states) = (result.0.unwrap(), result.1.unwrap());
        let model = model::Model::new_from_model_perf(perf, states, pbs, gbs);
        let n = seps[seps.len() - 1];
        let d = device::Devices::new(n, seps);
        SyncOrchestrate {
            m: model,
            d: d,
            res: vec![],
        }
    }

    /// Construct a SyncConductor from Model and Devices
    pub fn new_from_model_device(m: model::Model, d: device::Devices) -> SyncOrchestrate {
        SyncOrchestrate {
            m: m,
            d: d,
            res: vec![],
        }
    }

    fn init_matrix(&self, spa_size: u32, rp: u32, straight: bool) -> Matrix {
        // Shorthands
        let compute_times = &self.m.perf.compute_times;
        let activation_sizes = &self.m.perf.activation_sizes;
        let output_activation_sizes = &self.m.perf.output_activation_sizes;
        let parameter_sizes = &self.m.perf.parameter_sizes;
        let all_predecessor_ids = &self.m.perf.all_predecessor_ids;
        let mut A: Matrix = vec![];
        let d = &self.d;
        let num_machines = spa_size;
        let m_batch = self.m.global_batch_size / rp / self.m.min_micro_batch_size;
        let mb_f64 = m_batch as f64;

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
        for _ in 0..d.num_gpus {
            empty.push(false);
        }

        // DP Initialization
        let total_compute_time = compute_times[0][compute_times[0].len() - 1];
        let total_parameter_size = parameter_sizes[0][parameter_sizes[0].len() - 1];
        let total_dp_time = total_compute_time
            + data_parallel::all_reduce_time(d, &d.all_gpus(), total_parameter_size);
        if VERBOSE {
            println!("[orchestrate]\t Total DP Time = {:.7}", total_dp_time);
        }
        for j in 0..compute_times[0].len() {
            let cur_compute_time = compute_times[0][j];
            let cur_parameter_size = parameter_sizes[0][j];
            // let cur_activation_size = activation_sizes[0][j];
            let max_m = if straight { 1 } else { num_machines };
            for m in 0..max_m {
                if VERBOSE {
                    println!("[orchestrate]\t Assigning DP to A, m = {}, j = {}", m, j);
                }
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
                    if VERBOSE {
                        println!(
                            "[orchestrate]\t checking DP allreduce time for {:?}: {}",
                            &n.gids,
                            data_parallel::all_reduce_time(d, &n.gids, cur_parameter_size)
                        );
                    }
                    // calculate the current FPL [0 ... j][j ...] each with half machines
                    A[j][m as usize].get_mut().insert(
                        ph.clone(),
                        MatrixCell {
                            current_value: None,
                            current_maxmin_block: Some(
                                f64::max(
                                    (cur_compute_time) / (m + 1) as f64 / rp as f64 / mb_f64,
                                    0.0,
                                ) + data_parallel::all_reduce_time(d, &n.gids, cur_parameter_size),
                            ),
                            optimal_split: None,
                            num_gpus_used: Some(m + 1),
                            availability_bitset: empty.clone(),
                            gpu_ids: n.gids.clone(),
                        },
                    );
                    A[j][m as usize].get_mut().insert(
                        n.occupied.clone(),
                        MatrixCell {
                            current_value: None,
                            current_maxmin_block: Some(
                                f64::max(
                                    (cur_compute_time) / (m + 1) as f64 / rp as f64 / mb_f64,
                                    0.0,
                                ) + data_parallel::all_reduce_time(d, &n.gids, cur_parameter_size),
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

        A
    }

    /// Computations in the matrix to output raw planning metadata
    pub fn compute_plan_sync(&self, spa_size: u32, rp: u32, straight: bool) -> Matrix {
        // Shorthands
        let compute_times = &self.m.perf.compute_times;
        let output_activation_sizes = &self.m.perf.output_activation_sizes;
        let parameter_sizes = &self.m.perf.parameter_sizes;
        let all_predecessor_ids = &self.m.perf.all_predecessor_ids;
        let A: Matrix = self.init_matrix(spa_size, rp, straight);
        let d = &self.d;
        let num_machines = spa_size;
        let m_batch = self.m.global_batch_size / rp / self.m.min_micro_batch_size;
        let mb_f64 = m_batch as f64;
        let total_compute_time = compute_times[0][compute_times[0].len() - 1];

        // Bitset placeholder
        let mut ph: bitset = vec![];
        let mut empty: bitset = vec![];
        for _ in 0..num_machines * rp + 1 {
            ph.push(true);
        }
        for _ in 0..d.num_gpus {
            empty.push(false);
        }

        if m_batch == 0 {
            return A;
        }

        let min_m = 1;
        for m in min_m..num_machines {
            for j in 1..compute_times[0].len() {
                //                if VERBOSE {
                //                    println!("[orchestrate]\t pre-orchestration check, m = {}, j = {}", m, j);
                //                }
                //                if !A[j][m as usize].borrow().contains_key(&ph) && m > 0 {
                //                    continue;
                //                }
                if VERBOSE {
                    println!("[orchestrate]\t m = {}, j = {}", m, j);
                }

                let mut cur_A_bt = A[j][m as usize].borrow_mut();

                let empty_cell = MatrixCell {
                    current_value: None,
                    current_maxmin_block: None,
                    optimal_split: None,
                    num_gpus_used: None,
                    availability_bitset: vec![],
                    gpu_ids: BTreeSet::new(),
                };

                let cur_A: &MatrixCell;
                if cur_A_bt.contains_key(&ph) {
                    cur_A = cur_A_bt.get(&ph).unwrap();
                } else {
                    cur_A = &empty_cell;
                }

                // let cur_A = cur_A_bt.get(&ph).unwrap();
                let (
                    mut optimal_value,
                    mut min_pipeline_time,
                    mut optimal_split,
                    mut optimal_num_machines,
                    mut last_from,
                    mut last_machines,
                ) = (
                    cur_A.current_value,
                    cur_A.current_maxmin_block,
                    cur_A.optimal_split,
                    cur_A.num_gpus_used,
                    cur_A.availability_bitset.clone(),
                    cur_A.gpu_ids.clone(),
                );

                // trying [0 ... k] | [k+1 ... j]
                for k in all_predecessor_ids[j].iter() {
                    if VERBOSE {
                        println!("[orchestrate]\t m = {}, j = {}, k = {}", m, j, k);
                    }
                    let max_mp = if straight { 2 } else { m + 1 };
                    for mp in 1..max_mp {
                        for (bs, cell) in A[*k as usize][(m - mp) as usize].borrow().iter() {
                            // TODO: check this condition
                            if bs.len() as u32 > num_machines * rp {
                                continue; // skip ph
                            }

                            let next_bs_all = d.next_cards_with_replica(bs.to_vec(), mp, rp);

                            for nbs in next_bs_all {
                                let from = &cell.gpu_ids;
                                let to = &nbs.gids;

                                let mut input_transfer_time =
                                    split_concat::split_concat_all2all_time(
                                        d,
                                        from,
                                        to,
                                        2.0 * output_activation_sizes[*k as usize],
                                    );
                                input_transfer_time /= mb_f64;

                                let mut last_stage_time = compute_times[*k as usize + 1][j];
                                if last_stage_time < -0.5 {
                                    continue;
                                }
                                last_stage_time /= (mp * rp) as f64;
                                last_stage_time /= mb_f64;

                                // continue if A[k][m-mp][ph] does not exist
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

                                if VERBOSE {
                                    println!(
                                        "[orchestrate]\t last_stage_time for {},{},{},{} = {} | prev_time = {} | input_transfer_time = {}",
                                        m,
                                        j,
                                        k,
                                        mp,
                                        last_stage_time,
                                        A[*k as usize][(m - mp) as usize]
                                            .borrow()
                                            .get(bs)
                                            .unwrap()
                                            .current_maxmin_block
                                            .unwrap(),
                                        input_transfer_time
                                    );
                                }

                                // BLK max with previous DP result
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
                                    min_pipeline_time = Some(pipeline_time);
                                }

                                // FPL Check
                                let last_stage_allreduce = data_parallel::all_reduce_time(
                                    d,
                                    &nbs.gids,
                                    parameter_sizes[*k as usize + 1][j],
                                );
                                let cur_fpl = sync_pipeline::sync_pipeline_length_intermediate(
                                    min_pipeline_time.unwrap(),
                                    m_batch,
                                    compute_times[0][*k as usize] / mb_f64 / 2.0,
                                    total_compute_time / mb_f64 / 2.0,
                                    last_stage_allreduce,
                                );

                                // FPL Max with previous DP result

                                if !A[*k as usize][(m - mp) as usize].borrow().contains_key(&ph)
                                    || A[*k as usize][(m - mp) as usize]
                                        .borrow()
                                        .get(&ph)
                                        .unwrap()
                                        .current_value
                                        .is_none()
                                {
                                    continue;
                                }

                                let fpl_time = f64::max(
                                    A[*k as usize][(m - mp) as usize]
                                        .borrow()
                                        .get(bs)
                                        .unwrap()
                                        .current_value
                                        .unwrap(),
                                    cur_fpl,
                                );

                                if optimal_value.is_none() || fpl_time < optimal_value.unwrap() {
                                    if VERBOSE {
                                        println!(
                                            "[orchestrate]\t fpl_time {} < optimal_value, updating",
                                            fpl_time
                                        );
                                    }
                                    optimal_value = Some(fpl_time);
                                    optimal_split = Some((*k, m - mp));
                                    optimal_num_machines = Some(mp);
                                    last_from = bs.clone();
                                    last_machines = nbs.gids.clone();
                                }

                                if VERBOSE {
                                    println!(
                                        "[orchestrate]\t Current FPL = {}, FPL Time = {}",
                                        cur_fpl, fpl_time
                                    );
                                }

                                if !cur_A_bt.contains_key(&nbs.occupied)
                                    || cur_fpl
                                        < cur_A_bt
                                            .get(&nbs.occupied)
                                            .unwrap()
                                            .current_value
                                            .unwrap()
                                {
                                    if VERBOSE {
                                        println!(
                                            "[orchestrate]\t Updating A[{}][{}][{:?}] \t| value: {:.7}\t maxmin_block: {:.7}\t split: {:?}\t from_bs: {:?}\t gids: {:?} ",
                                            j,
                                            m,
                                            &nbs.occupied.iter().fold(String::new(), |acc, &b| acc
                                                + &(b as i32).to_string()),
                                            fpl_time,
                                            pipeline_time,
                                            (*k, m - mp),
                                            &bs.iter().fold(String::new(), |acc, &b| acc
                                                + &(b as i32).to_string()),
                                            nbs.gids.clone(),
                                        );
                                    }
                                    cur_A_bt.insert(
                                        nbs.occupied.clone(),
                                        MatrixCell {
                                            current_value: Some(fpl_time),
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
                        current_value: optimal_value,
                        current_maxmin_block: min_pipeline_time,
                        optimal_split: optimal_split,
                        num_gpus_used: optimal_num_machines,
                        availability_bitset: last_from.clone(),
                        gpu_ids: last_machines.clone(),
                    },
                );
            }
        }
        A
    }

    /// Analyse the raw data from compute_plan_sync and output a human-readable plan
    pub fn analyse_plan_sync(
        &self,
        A: &Matrix,
        end: u32,
        num_machines: u32,
        rp: u32,
    ) -> Vec<(u32, u32, u32, BTreeSet<u32>)> {
        let mut res: Vec<(u32, u32, u32, BTreeSet<u32>)> = vec![];
        let mut ph: bitset = vec![];
        for _ in 0..num_machines * rp + 1 {
            ph.push(true);
        }
        let mut mt = A[end as usize - 1][num_machines as usize - 1].borrow();
        let mut metadata = mt.get(&ph).unwrap();

        let mut next_split = metadata.optimal_split;
        let mut last_machines = metadata.gpu_ids.clone();
        if last_machines.is_empty() {
            println!("Last Machines is EMPTY! \nFinal Context Matrix\n");
            SyncOrchestrate::print_matrix(A);
            panic!("last_machines.is_empty()");
        }
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

            mt = A[next_split.unwrap().0 as usize][next_split.unwrap().1 as usize].borrow();
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

    /// Shorthand for Planning each individual pipeline spin
    pub fn plan_for(&self, i: u32, straight: bool) -> SyncOrchestrateResult {
        let num_gpus = self.d.num_gpus;
        // 2 <= i <= num_gpus
        let rp = num_gpus / i;
        println!("Planning for {} x {}, {}", i, rp, straight);
        let A = self.compute_plan_sync(i, rp, straight);
        println!("Planning Done");

        let mut ph: bitset = vec![];
        for _ in 0..i * rp + 1 {
            ph.push(true);
        }

        if VERBOSE {
            SyncOrchestrate::print_matrix(&A);
        }

        let pipeline_block_bt = A[self.m.perf.compute_times[0].len() - 1][i as usize - 1].borrow();
        let pipeline_block = pipeline_block_bt.get(&ph).unwrap();
        let pipeline_time = pipeline_block.current_maxmin_block.unwrap();
        if pipeline_time < 0.001 {
            panic!("ppl time error");
        }
        println!("Pipeline Time: {}", pipeline_time);
        let res = self.analyse_plan_sync(&A, self.m.perf.compute_times[0].len() as u32, i, rp);
        let res_speedup = sync_pipeline::sync_pipeline_speedup_analytical(
            &self.d,
            &self.m,
            rp,
            pipeline_time,
            res.clone(),
        );
        SyncOrchestrateResult {
            speedup: res_speedup,
            stages: res,
        }
    }

    pub fn print_matrix(A: &Matrix) {
        for j in 0..A.len() {
            for m in 0..A[j].len() {
                for (k, v) in A[j][m].borrow().iter() {
                    println!(
                        "A[{}][{}][{:?}] \t| value: {:.7?}\t maxmin_block: {:.7?}\t split: {:?}\t from_bs: {:?}\t gids: {:?} ",
                        j,
                        m,
                        k.iter().fold(String::new(), |acc, &b| acc
                            + &(b as i32).to_string()),
                        v.current_value,
                        v.current_maxmin_block,
                        v.optimal_split,
                        v.availability_bitset.iter().fold(String::new(), |acc, &b| acc
                            + &(b as i32).to_string()),
                        v.gpu_ids,
                    );
                }
            }
        }
    }
}

/// SyncConductor Python API
#[pymethods]
impl SyncOrchestrate {
    fn py_plan_for(
        &self,
        i: u32,
        straight: bool,
    ) -> PyResult<(f64, Vec<(u32, u32, u32, Vec<u32>)>)> {
        let res = self.plan_for(i, straight);
        let speedup = res.speedup;
        let py_stages: Vec<(u32, u32, u32, Vec<u32>)>;
        py_stages = res
            .stages
            .iter()
            .map(|s| (s.0, s.1, s.2, s.3.iter().cloned().collect()))
            .collect();
        Ok((speedup, py_stages))
    }
}

impl OrchestrationResult for SyncOrchestrateResult {
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

impl Orchestrate for SyncOrchestrate {
    fn orchestrate(&mut self) {
        let num_gpus = self.d.num_gpus;
        let vec_range: Vec<u32> = (2..num_gpus + 1).collect();
        let result: Vec<_> = vec_range
            .par_iter()
            .map(|i| self.plan_for(*i, false))
            .collect();

        /*
        let mut straight_vec: Vec<u32> = vec![];
        if num_gpus > 3 {
            straight_vec.push(2); // 1:1
            straight_vec.push(3); // 1:1:1
            straight_vec.push(num_gpus); // all straight
        } else if num_gpus > 2 {
            straight_vec.push(2); // 1:1
            straight_vec.push(num_gpus); // all straight
        } else {
            straight_vec.push(num_gpus); // all straight
        }

        // println!("Appending Orchestration for {:?}", straight_vec);

        let result_straight: Vec<_> = straight_vec
            .par_iter()
            .map(|i| self.plan_for(*i, true))
            .collect();

        result.extend(result_straight);
        */

        self.res = result;
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
