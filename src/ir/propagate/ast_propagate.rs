use crate::ir::error::PropagationError::*;
use crate::ir::hlo_ast::{HLOFunction, Instruction, Param};
use log::debug;
use petgraph::prelude::*;
use rayon::prelude::*;
