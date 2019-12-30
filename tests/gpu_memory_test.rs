extern crate HPGO;
extern crate rayon;
use rayon::prelude::*;
use HPGO::analysis::*;
use HPGO::environment::*;
use HPGO::input::*;
use HPGO::model::*;

#[test]
fn test_model_max_batch_size() {
    let models = vec![
        // ("vgg16", 32),
        // ("vgg19", 32),
        // ("xlnet", 1),
        // ("amoebanet", 8),
        ("bert_large", 2, 3, 1733171968.0),
        // ("resnet50", 32),
    ];

    let res: Vec<_> = models
        .par_iter()
        .map(|(s, bs, opt_scale, papb)| {
            let tgi: torch_graph::TorchGraphImporter = ModelImporter::new();
            let result = tgi.ImportFrom(&["./profiles/", s, "/graph.txt"].join(""));
            let (perf, states) = (result.0.unwrap(), result.1.unwrap());
            let mut model = model::Model::new_from_model_perf(perf, states, *bs, 6);
            model.set_optimizer_memory_scaling(*opt_scale);
            if *papb > 0.0 {
                model.set_peak_activation_per_batch(*papb);
            }
            (s, gpu_memory::max_single_gpu_batch_size(&model))
        })
        .collect();

    println!();
    for i in res {
        println!("{}: Max Single GPU Batch Size {}", i.0, i.1);
    }
}
