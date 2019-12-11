
pub struct ModelPerf {
    compute_times: Vec<Vec<f64>>,
    activation_sizes: Vec<Vec<f64>>,
    parameter_sizes: Vec<Vec<f64>>,
    output_activation_sizes: Vec<f64>,
    all_predecessor_ids: Vec<Vec<u32>>,
}

