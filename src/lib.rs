//! # HPGO: Hybrid Parallelism Global Orchestration
//! This library provides

#![allow(non_snake_case)]

extern crate itertools;
extern crate ordered_float;
extern crate pyo3;
extern crate rayon;
extern crate serde;
extern crate serde_json;
extern crate pest;
extern crate log;
extern crate fern;


/// HPGO Hardware Environment: Ethernet, GPU, NVLink, etc.
pub mod environment;

/// HPGO Model Importer, currently TorchGraphImporter only
pub mod input;

/// HPGO Layerwise Model Abstraction
pub mod layerwise;

/// HPGO IR Abstraction
pub mod ir;
//
// /// HPGO Orchestration Variations
// pub mod orchestration;
//
// /// HPGO Parallelism Definitions and Helpers
// pub mod parallelism;

/// HPGO Public API: C & Python
pub mod api;
