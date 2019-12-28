extern crate HPGO;
use HPGO::environment::network;
use HPGO::input::*;
use HPGO::orchestration::*;

#[test]
fn test_orchestrate_compute_plan() {
    let mut c = orchestrate::SyncConductor::new(
        "./profiles/amoebanet/flattened.txt",
        [1, 2, 3, 4, 5, 6, 7, 8].to_vec(),
    );
    c.compute_plan_sync(8, 1);
    println!("\nFinal Context Matrix\n");
    for j in 0..c.A.len() {
        for m in 0..c.A[j].len() {
            for (k, v) in c.A[j][m].borrow().iter() {
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
    let mut c =
        orchestrate::SyncConductor::new("./profiles/amoebanet/flattened.txt", [8, 16].to_vec());
    c.compute_plan_sync(16, 1);
    let res = c.analyse_plan_sync(c.m.perf.compute_times[0].len() as u32, 16, 1);
    println!("{:?}", res);
}
