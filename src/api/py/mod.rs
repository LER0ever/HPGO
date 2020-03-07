use environment::*;
use input::*;
use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, wrap_pymodule};

pub mod py_layerwise;
pub mod py_ir;

use api::py::py_layerwise::*;
use api::py::py_ir::*;

#[pymodule]
fn HPGO(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__title__", "HPGO")?;
    m.add("__doc__", "Hybrid Parallelism Global Orchestration")?;
    m.add("__author__", "Yi Rong <hi@rongyi.io>")?;
    m.add("__copyright__", "Copyright 2020 Yi Rong")?;
    m.add("__license__", "BSD-3-Clause")?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_wrapped(wrap_pymodule!(Layerwise))?;
    m.add_wrapped(wrap_pymodule!(IR))?;

    Ok(())
}
