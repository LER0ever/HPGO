use crate::ir::error::DeriveError::*;
use std::collections::HashMap;
use std::error::Error;

use rayon::prelude::*;

use crate::ir::hlo_ast::*;

pub type Split<'a> = (&'a str, i8);

pub struct Derivation<'a> {
    pub derive_cache: HashMap<&'a Instruction, HashMap<&'a str, i8>>,
}

impl<'a> Derivation<'a> {
    pub fn cache_all_derive(&self, ast_root: &'a HLORoot) -> Result<(), Box<dyn Error>> {
        let ok = ast_root
            .functions
            .par_iter()
            .map(|f| {
                f.body
                    .par_iter()
                    .map(|i| self.cache_derive(i).is_ok())
                    .all(|x| x == true)
            })
            .all(|x| x == true);
        match ok {
            true => Ok(()),
            false => Err("Caching derivation has at least one failure...".into()),
        }
    }

    pub fn cache_derive(&self, inst: &'a Instruction) -> Result<(), Box<dyn Error>> {
        match inst.function.name {
            _ => Ok(()),
        }
    }

    fn d_matmul(&self, inst: &'a Instruction) -> Result<Vec<HashMap<&'a str, i8>>, Box<dyn Error>> {
        inst.assert_param_len(2);
        inst.assert_key_in_meta("lhs_contracting_dims");
        inst.assert_key_in_meta("rhs_contracting_dims");
        let lhs_contracting_dims = inst.get_meta_vec("lhs_contracting_dims")?;
        let rhs_contracting_dims = inst.get_meta_vec("rhs_contracting_dims")?;
        let mut lhs_batch_dims = None;
        let mut rhs_batch_dims = None;
        if inst.key_in_meta("lhs_batch_dims") {
            lhs_batch_dims = Some(inst.get_meta_vec("lhs_batch_dims")?);
            rhs_batch_dims = Some(inst.get_meta_vec("rhs_batch_dims")?);
        }

        let mut result: Vec<HashMap<&'a str, i8>> = vec![];

        // split into contracting dim
        {
            assert_eq!(lhs_contracting_dims.len(), 1);
            assert_eq!(rhs_contracting_dims.len(), 1);
            let mut m: HashMap<&'a str, i8> = HashMap::new();
            let params = inst
                .function
                .params
                .as_ref()
                .ok_or(OptionNone("inst.fn.params".into()))?;
            let first_param: &'a str = &params[0].name;
            m.insert(first_param, lhs_contracting_dims[0] as i8);
            result.push(m);
        }

        unimplemented!()
    }
}
