extern crate HPGO;
extern crate rayon;
use rayon::prelude::*;
use HPGO::analysis::*;
use HPGO::environment::*;
use HPGO::input::*;
use HPGO::model::*;

fn t_p3_for(filename: &str) -> cc_overlap::OverlapStats {
    let d16 = device::Devices::new(16, vec![8, 16]);
    let tgi: torch_graph::TorchGraphImporter = ModelImporter::new();
    let result = tgi.ImportFrom(filename);
    let (perf, states) = (result.0.unwrap(), result.1.unwrap());
    let model = model::Model::new_from_model_perf(perf, states);
    cc_overlap::p3(&d16, &model)
}

#[test]
fn test_p3_for_all() {
    let models = vec![
        "vgg16",
        "vgg19",
        "xlnet",
        "amoebanet",
        "bert_large",
        "resnet50",
        "gnmt_large",
    ];
    let res: Vec<_> = models
        .par_iter()
        .map(|&s| (s, t_p3_for(&["./profiles/", s, "/graph.txt"].join(""))))
        .collect();
    println!();
    for i in res {
        println!("{}: P3 Speedup {:.5}%", i.0, i.1.speedup_percentage);
    }
}
