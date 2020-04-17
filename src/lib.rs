//! # HPGO: Hybrid Parallelism Global Orchestration
//! This library provides

#![allow(non_snake_case)]
#![feature(atomic_min_max)]

/// HPGO Hardware Environment: Ethernet, GPU, NVLink, etc.
pub mod environment;

/// HPGO Model Importer, currently TorchGraphImporter only
pub mod input;

/// HPGO Layerwise Model Abstraction
pub mod layerwise;

/// HPGO IR Abstraction
pub mod ir;

/// HPGO Public API: C & Python
pub mod api;
