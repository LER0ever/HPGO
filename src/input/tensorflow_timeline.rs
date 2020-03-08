use super::LayerwiseModelImporter;
use crate::layerwise::model::model_perf;

struct TensorflowTimelineImporter {}

impl LayerwiseModelImporter for TensorflowTimelineImporter {
    fn new() -> TensorflowTimelineImporter {
        TensorflowTimelineImporter {}
    }
    fn ImportFrom(
        &self,
        _filename: &str,
    ) -> (
        Option<model_perf::ModelPerf>,
        Option<model_perf::ModelStates>,
    ) {
        unimplemented!();
    }
}
