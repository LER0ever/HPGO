extern crate HPGO;
use HPGO::input::*;
use HPGO::orchestration::*;

#[test]
fn test_orchestrate_hierarchical_compute_plan() {
    let tgi: torch_graph::TorchGraphImporter = ModelImporter::new();
    let result = tgi.ImportFrom("./profiles/xlnet/graph.txt");
}
