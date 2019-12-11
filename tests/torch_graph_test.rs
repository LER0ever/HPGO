extern crate HPGO;
use HPGO::input::*;

#[test]
fn test_python_env() {
    let tgi: torch_graph::TorchGraphImporter = ModelImporter::new();
    tgi.ImportFrom("./profiles/xlnet/graph.txt");
}