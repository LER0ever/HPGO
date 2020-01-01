use input::*;
use model::*;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
/// Formats the sum of two numbers as string
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn model_from_torch_graph(filename: &str, pbs: u32, gbs: u32) -> PyResult<model::Model> {
    let tgi: torch_graph::TorchGraphImporter = ModelImporter::new();
    let result = tgi.ImportFrom(filename);
    let (perf, states) = (result.0.unwrap(), result.1.unwrap());
    let mut model = model::Model::new_from_model_perf(perf, states, pbs, gbs);
    Ok(model)
}

#[pymodule]
fn HPGO(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(sum_as_string))?;
    m.add_wrapped(wrap_pyfunction!(model_from_torch_graph))?;

    Ok(())
}
