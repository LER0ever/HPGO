use crate::ir::hlo_ast::HLORoot;
use pyo3::prelude::*;

#[pymodule]
fn IR(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
