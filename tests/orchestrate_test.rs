extern crate HPGO;
use HPGO::environment::*;
use HPGO::input::*;
use HPGO::model::*;
use HPGO::orchestration::*;

#[test]
fn test_orchestrate_compute_plan() {
    let mut c = orchestrate::SyncConductor::new_from_torch_graph(
        "./profiles/amoebanet/flattened.txt",
        8,
        1024,
        [1, 2, 3, 4, 5, 6, 7, 8].to_vec(),
    );
    let A = c.compute_plan_sync(8, 1);
    println!("\nFinal Context Matrix\n");
    for j in 0..A.len() {
        for m in 0..A[j].len() {
            for (k, v) in A[j][m].borrow().iter() {
                // println!(
                //     "{} {} {:?} | {:?}",
                //     j,
                //     m,
                //     k.iter()
                //         .fold(String::new(), |acc, &b| acc + &(b as i32).to_string()),
                //     v
                // );
                println!(
                    "A[{}][{}][{:?}] \t| maxmin_block: {:.7?}\t split: {:?}\t from_bs: {:?}\t gids: {:?} ",
                    j,
                    m,
                    k.iter().fold(String::new(), |acc, &b| acc
                        + &(b as i32).to_string()),
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

#[test]
fn test_orchestrate_analyse_plan() {
    let mut c = orchestrate::SyncConductor::new_from_torch_graph(
        "./profiles/amoebanet/flattened.txt",
        8,
        1024,
        [8, 16].to_vec(),
    );
    let A = c.compute_plan_sync(16, 1);
    let res = c.analyse_plan_sync(&A, c.m.perf.compute_times[0].len() as u32, 16, 1);
    println!("{:?}", res);
}

#[test]
fn test_orchestrate_orchestrate() {
    // Construct Model
    let tgi: torch_graph::TorchGraphImporter = ModelImporter::new();
    let result = tgi.ImportFrom(&["./profiles/", "xlnet", "/graph.txt"].join(""));
    let (perf, states) = (result.0.unwrap(), result.1.unwrap());
    let mut model = model::Model::new_from_model_perf(perf, states, 1, 256);
    model.set_optimizer_memory_scaling(3);
    model.set_peak_activation_per_batch(3942774528.0);
    model.set_min_microbatch_size(1);
    // Construct Devices
    let d16 = device::Devices::new(16, vec![8, 16]);

    let mut c = orchestrate::SyncConductor::new_from_model_device(model, d16);
    c.orchestrate();
}
