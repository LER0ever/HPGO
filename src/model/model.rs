use model::model_perf;

#[derive(Debug)]
pub struct Layer {
   pub id: u32,
   pub name: Option<String>,
   pub desc: Option<String>,
   pub compute_time: f64,
   pub activation_size: f64,
   pub output_activation_size: f64,
   pub parameter_size: f64,
}

#[derive(Debug)]
pub struct Model {
    pub layers: Vec<Layer>,
    pub perf: model_perf::ModelPerf,
    pub global_batch_size: u32,
    pub profile_batch_size: u32,
    pub min_micro_batch_size: u32,
    pub use_async: bool,
}

impl Model {
    fn new() -> Model {
        println!("For now, don't call new(), construct from perf results instead");
        panic!()
    }
    fn new_from_model_perf(perf: model_perf::ModelPerf) -> Model {
        // WIP
        unimplemented!()
    }
}