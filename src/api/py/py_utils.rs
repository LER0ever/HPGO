use crate::layerwise::parallelism::sync_pipeline::sync_pipeline_length_recursive;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn pipeline_length_recursive(F: Vec<f64>, B: Vec<f64>, AllReduce: Vec<f64>, M: u32, mut Phi: Option<Vec<u32>>) -> PyResult<f64> {
    // Phi default to 3,2,1...
    if Phi.is_none() {
        let S = F.len();
        let mut phi: Vec<u32> = vec![];
        for i in 0..S {
            phi.push((S-i+1) as u32);
        }
        Phi = Some(phi)
    }
    let phi = Phi.unwrap();
    let result = sync_pipeline_length_recursive(F, B, M, phi, AllReduce);
    Ok(result)
}

#[pymodule]
fn Utils(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_wrapped(wrap_pyfunction!(pipeline_length_recursive))?;

    Ok(())
}
