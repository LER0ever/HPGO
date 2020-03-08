// import from PyTorch Profiler graph
pub mod torch_graph;
mod torch_graph_py;

// import from TensorFlow Timeline
pub mod tensorflow_timeline;

// import from HLOComputation.to_string() result
pub mod hlo_string;

use layerwise::model::model_perf;
use ir::hlo_ast::HLORoot;
use std::error::Error;

pub trait LayerwiseModelImporter {
    fn new() -> Self;
    fn ImportFrom(
        &self,
        filename: &str,
    ) -> (
        Option<model_perf::ModelPerf>,
        Option<model_perf::ModelStates>,
    );
}

pub trait HLOModelImporter {
    fn new() -> Self;
    fn ImportFrom(
        &self,
        filename: &str,
    ) -> Result<HLORoot, Box<dyn Error>>;
}

pub trait DAGModelImporter {
    fn new() -> Self;
    fn ImportFrom(
        &self,
        filename: &str,
    ) -> ();
}
