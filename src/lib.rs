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

pub mod environment;
pub mod input;
pub mod model;
pub mod orchestration;
pub mod parallelism;
