use HPGO::input::*;

#[test]
fn test_torch_python_env() {
    // should not crash
    let tgi: torch_graph::TorchGraphImporter = LayerwiseModelImporter::new();
    tgi.ImportFrom("./profiles/xlnet/graph.txt");
}

#[test]
fn test_torch_python_import_basic() {
    let tgi: torch_graph::TorchGraphImporter = LayerwiseModelImporter::new();
    let result = tgi.ImportFrom("./profiles/vgg16/graph.txt");
    match result {
        (Some(x), Some(_y)) => {
            println!("Got result successfully, printing all fields...");
            // NOTE: could've just print x, as it derives Debug
            println!("compute_times: {:?}", x.compute_times);
            println!("activation_sizes: {:?}", x.activation_sizes);
            println!("parameter_sizes: {:?}", x.parameter_sizes);
            println!("output_activation_sizes: {:?}", x.output_activation_sizes);
            println!("all_predecessor_ids: {:?}", x.all_predecessor_ids);

            // println!("model_states: {:?}, {:?}", y.len(), x.compute_times[0].len())
        }
        _ => {
            panic!();
        }
    }
}

#[test]
fn test_hlo_import_basic_json() {
    let hi: hlo_string::HLOStructuredJsonImporter = HLOModelImporter::new();
    let result = hi
        .ImportFrom("./tests/test_data/hlo/elemwise.json")
        .unwrap();
    println!("{:#?}", result);
}

#[test]
fn test_hlo_import_full_json() {
    let hi: hlo_string::HLOStructuredJsonImporter = HLOModelImporter::new();
    let _result = hi.ImportFrom("./tests/test_data/hlo/hlo.json").unwrap();
    // as long as unwrap succeeds
    // println!("{:#?}", result);
}
