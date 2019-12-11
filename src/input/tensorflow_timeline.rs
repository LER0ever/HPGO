use model::model_perf;
use super::ModelImporter;

struct TensorflowTimelineImporter {
}

impl ModelImporter for TensorflowTimelineImporter {
    fn new() -> TensorflowTimelineImporter {
        TensorflowTimelineImporter{}
    }
    fn ImportFrom(&self, filename: &str) -> Option<model_perf::ModelPerf> {
        unimplemented!();
    }
}
