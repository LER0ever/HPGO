use model::model_perf;

#[derive(Debug)]
pub struct Layer {
    pub id: Option<u32>,
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
    pub states: model_perf::ModelStates,
    pub global_batch_size: u32,
    pub profile_batch_size: u32,
    pub min_micro_batch_size: u32,
    pub use_async: bool,
}

impl Model {
    pub fn new() -> Model {
        println!("For now, don't call new(), construct from perf results instead");
        panic!()
    }
    pub fn new_from_model_perf(perf: model_perf::ModelPerf, states: model_perf::ModelStates) -> Model {
        // WIP
        let mut layers: Vec<Layer> = vec![];
        for i in 0..perf.compute_times[0].len() {
            layers.push(Layer {
                id: Some(i as u32),
                name: None,
                desc: None,
                compute_time: perf.compute_times[i][i],
                activation_size: perf.activation_sizes[i][i],
                output_activation_size: perf.output_activation_sizes[i],
                parameter_size: perf.parameter_sizes[i][i],
            });
        }
        Model {
            layers: layers,
            perf: perf,
            states: states,
            global_batch_size: 0,
            profile_batch_size: 0,
            min_micro_batch_size: 0,
            use_async: false,
        }
    }
}
