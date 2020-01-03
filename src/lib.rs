//! HPGO!

#![allow(non_snake_case)]
extern crate csv;
extern crate ndarray;
#[macro_use]
extern crate itertools;
extern crate float_cmp;
extern crate ordered_float;
#[macro_use]
extern crate serde;
extern crate strsim;

extern crate pyo3;
extern crate rayon;

/// HPGO Model Analysis
pub mod analysis;

/// HPGO Hardware Environment: Ethernet, GPU, NVLink, etc.
pub mod environment;

/// HPGO Model Importer, currently TorchGraphImporter only
pub mod input;

/// HPGO Model Abstract Definition
pub mod model;

/// HPGO Orchestration Variations
pub mod orchestration;

/// HPGO Parallelism Definitions and Helpers
pub mod parallelism;

/// HPGO Python API
pub mod pylib;
