use crate::environment::*;
use crate::input::*;
use crate::layerwise::model::*;
use crate::layerwise::orchestration::*;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn model_from_torch_graph(filename: &str, pbs: u32, gbs: u32) -> PyResult<model::Model> {
    let tgi: torch_graph::TorchGraphImporter = LayerwiseModelImporter::new();
    let result = tgi.ImportFrom(filename);
    let (perf, states) = (result.0.unwrap(), result.1.unwrap());
    let model = model::Model::new_from_model_perf(perf, states, pbs, gbs);
    Ok(model)
}

#[pyfunction]
fn devices_from_seps(seps: Vec<u32>) -> PyResult<device::Devices> {
    assert_eq!(seps.iter().max().unwrap(), &seps[seps.len() - 1]);
    let d = device::Devices::new(seps[seps.len() - 1], seps);
    Ok(d)
}

#[pyfunction]
fn conductor_from_torch_graph_and_seps(
    filename: &str,
    pbs: u32,
    gbs: u32,
    seps: Vec<u32>,
) -> PyResult<orchestrate_async::AsyncOrchestrate> {
    let tgi: torch_graph::TorchGraphImporter = LayerwiseModelImporter::new();
    let result = tgi.ImportFrom(filename);
    let (perf, states) = (result.0.unwrap(), result.1.unwrap());
    let m = model::Model::new_from_model_perf(perf, states, pbs, gbs);

    assert_eq!(seps.iter().max().unwrap(), &seps[seps.len() - 1]);
    let d = device::Devices::new(seps[seps.len() - 1], seps);

    let c = orchestrate_async::AsyncOrchestrate::new_from_model_device(m, d);

    Ok(c)
}

#[pymodule]
fn Layerwise(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_wrapped(wrap_pyfunction!(model_from_torch_graph))?;
    m.add_wrapped(wrap_pyfunction!(devices_from_seps))?;
    m.add_wrapped(wrap_pyfunction!(conductor_from_torch_graph_and_seps))?;

    Ok(())
}
