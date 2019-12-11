#[derive(Debug)]
pub struct ModelPerf {
    pub compute_times: Vec<Vec<f64>>,
    pub activation_sizes: Vec<Vec<f64>>,
    pub parameter_sizes: Vec<Vec<f64>>,
    pub output_activation_sizes: Vec<f64>,
    pub all_predecessor_ids: Vec<Vec<u32>>,
}
