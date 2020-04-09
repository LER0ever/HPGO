use crate::ir::conductor::IRConductor;
use crate::ir::hlo_ast::HLORoot;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// #[pyfunction]
// fn new_from_hlo_json(filename: String) -> PyResult<IRConductor> {
//     let mut irc = IRConductor::new();
//     irc.import_from(filename).unwrap();
//     Ok(irc)
// }

#[pymodule]
fn IR(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<IRConductor>()?;
    // m.add_wrapped(wrap_pyfunction!(new_from_hlo_json))?;

    Ok(())
}
