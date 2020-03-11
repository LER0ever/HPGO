use crate::ir::derive::derivation::Derivation;
use crate::ir::error::DeriveError::*;
use crate::ir::hlo_ast::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::error::Error;

impl<'a> Derivation<'a> {
    pub fn d(&self, inst: &'a Instruction) -> Result<Vec<HashMap<&'a str, i8>>, Box<dyn Error>> {
        match inst.function.name.as_str() {
            "dot" => self.d_matmul(inst),
            "add" | "and" | "divide" | "subtract" | "multiply" | "maximum" | "abs" | "negate"
            | "sine" | "cosine" | "sqrt" | "rsqrt" | "log" | "exponential" | "convert"
            | "compare" | "all-reduce" => self.d_elem(inst),
            _ => Err("asdf".into()),
        }
    }

    fn add_keys(
        r: &mut Vec<HashMap<&'a str, i8>>,
        kv: Vec<(&'a str, i8)>,
    ) -> Result<(), Box<dyn Error>> {
        let mut m: HashMap<&'a str, i8> = HashMap::new();
        m.par_extend(kv.into_par_iter());
        r.push(m);
        Ok(())
    }

    fn d_matmul(&self, inst: &'a Instruction) -> Result<Vec<HashMap<&'a str, i8>>, Box<dyn Error>> {
        inst.assert_param_len(2);
        inst.assert_key_in_meta("lhs_contracting_dims");
        inst.assert_key_in_meta("rhs_contracting_dims");
        let lhs_contracting_dims = inst.get_meta_vec("lhs_contracting_dims")?;
        let rhs_contracting_dims = inst.get_meta_vec("rhs_contracting_dims")?;
        assert_eq!(lhs_contracting_dims.len(), 1);
        assert_eq!(rhs_contracting_dims.len(), 1);
        let mut lhs_batch_dims = None;
        let mut rhs_batch_dims = None;
        if inst.key_in_meta("lhs_batch_dims") {
            lhs_batch_dims = Some(inst.get_meta_vec("lhs_batch_dims")?);
            rhs_batch_dims = Some(inst.get_meta_vec("rhs_batch_dims")?);
        }

        let all_dims: Vec<i32> = (0..(inst.function.return_types[0]
            .dimensions
            .as_ref()
            .ok_or(OptionNone("inst.fn.return_type.dimensions".into()))?
            .len()) as i32)
            .collect();
        let mat_dims: Vec<i32> = all_dims
            .par_iter()
            .filter(|x| !lhs_batch_dims.unwrap_or(&vec![]).contains(x))
            .map(|x| *x)
            .collect();

        let mut result: Vec<HashMap<&'a str, i8>> = vec![];
        let params = &inst
            .function
            .params
            .as_ref()
            .ok_or(OptionNone("inst.fn.params".into()))?;
        let first_param: &'a str = &params[0].name;
        let second_param: &'a str = &params[1].name;
        let var_name: &'a str = &inst.var_name;

        // All Replicate
        Self::add_keys(
            &mut result,
            vec![(first_param, -1i8), (second_param, -1i8), (var_name, -1i8)],
        )?;

        // split into contracting dim
        Self::add_keys(
            &mut result,
            vec![
                (first_param, lhs_contracting_dims[0] as i8),
                (second_param, rhs_contracting_dims[0] as i8),
                (var_name, -1i8),
            ],
        )?;

        // split into non-contract for first
        Self::add_keys(
            &mut result,
            vec![
                (
                    first_param,
                    mat_dims
                        .iter()
                        .filter(|x| !lhs_contracting_dims.contains(x))
                        .map(|x| *x as i8)
                        .next()
                        .ok_or(OptionNone("mat_dims - lhs_contracting_dims next()".into()))?,
                ),
                (second_param, -1i8),
                (var_name, mat_dims[0] as i8),
            ],
        )?;

        // split into non-contract for second
        Self::add_keys(
            &mut result,
            vec![
                (
                    second_param,
                    mat_dims
                        .iter()
                        .filter(|x| !rhs_contracting_dims.contains(x))
                        .map(|x| *x as i8)
                        .next()
                        .ok_or(OptionNone("mat_dims - rhs_contracting_dims next()".into()))?,
                ),
                (first_param, -1i8),
                (var_name, mat_dims[1] as i8),
            ],
        )?;

        // split into batch dims
        if lhs_batch_dims.is_some() && rhs_batch_dims.is_some() {
            lhs_batch_dims
                .unwrap()
                .iter()
                .enumerate()
                .for_each(|(i, x)| {
                    assert_eq!(lhs_batch_dims.unwrap()[i], rhs_batch_dims.unwrap()[i]);
                    Self::add_keys(
                        &mut result,
                        vec![
                            (first_param, *x as i8),
                            (second_param, *x as i8),
                            (var_name, *x as i8),
                        ],
                    )
                    .unwrap(); // TODO: refactor to for loop and use ?
                });
        }

        Ok(result)
    }

    fn d_elem(&self, inst: &'a Instruction) -> Result<Vec<HashMap<&'a str, i8>>, Box<dyn Error>> {
        inst.function
            .params
            .as_ref()
            .ok_or(OptionNone("inst.fn.params".into()))?
            .par_iter()
            .for_each(|x| {
                assert_eq!(
                    x.param_type.dimensions.as_ref().unwrap_or(&vec![]),
                    inst.function.return_types[0]
                        .dimensions
                        .as_ref()
                        .unwrap_or(&vec![])
                );
            });

        let mut result: Vec<HashMap<&'a str, i8>> = vec![];
        let params = inst
            .function
            .params
            .as_ref()
            .ok_or(OptionNone("inst.fn.params".into()))?;
        let var_name: &'a str = &inst.var_name;

        let mut all_dims: Vec<i32> = (0..inst.function.return_types[0]
            .dimensions
            .as_ref()
            .unwrap_or(&vec![])
            .len() as i32)
            .collect();
        all_dims.push(-1);

        // iterate over all dimensions index, including -1 for replication
        for d in all_dims {
            let mut splits: Vec<(&'a str, i8)> = vec![(var_name, d as i8)];
            for p in params {
                splits.push((&p.name, d as i8))
            }
            Self::add_keys(&mut result, splits)?;
        }

        Ok(result)
    }
}
