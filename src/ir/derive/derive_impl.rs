use crate::ir::derive::derivation::Derivation;
use crate::ir::error::DeriveError::*;
use crate::ir::hlo_ast::*;
use log::debug;
use rayon::prelude::*;
use std::collections::HashMap;
use std::error::Error;

impl<'a> Derivation<'a> {
    pub fn d(inst: &'a Instruction) -> Result<Vec<HashMap<&'a str, i8>>, Box<dyn Error>> {
        match inst.function.name.as_str() {
            "dot" => Self::d_matmul(inst),
            "add" | "and" | "divide" | "subtract" | "multiply" | "maximum" | "abs" | "negate"
            | "sine" | "cosine" | "sqrt" | "rsqrt" | "log" | "exponential" | "convert"
            | "compare" | "all-reduce" | "select" => Self::d_elem(inst),
            "reshape" => Self::d_reshape_alt(inst),
            "parameter" | "constant" | "copy" | "rng" | "iota" | "tuple" => Self::d_identity(inst),
            "reduce" => Self::d_reduce(inst),
            "transpose" => Self::d_transpose(inst),
            "gather" => Self::d_gather(inst),
            "scatter" => Self::d_scatter(inst),
            "slice" | "pad" => Self::d_pad_slice(inst),
            "concatenate" => Self::d_concat(inst),
            "padded_where" => Self::d_padded_where(inst),
            "broadcast" => Self::d_broadcast(inst),
            _ => {
                // println!(
                //     "Unimplemented derivation for fn {}, falling back to replication",
                //     inst.function.name.as_str()
                // );
                Self::d_replicate(inst)
            }
            // _ => Err(Box::new(DerivationUnimplemented(
            //     inst.function.name.clone(),
            // ))),
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

    fn d_matmul(inst: &'a Instruction) -> Result<Vec<HashMap<&'a str, i8>>, Box<dyn Error>> {
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
        let params = inst.get_all_params()?;
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

    fn d_elem(inst: &'a Instruction) -> Result<Vec<HashMap<&'a str, i8>>, Box<dyn Error>> {
        // assert length TODO
        // assert all param of the same shape, remove later
        inst.get_all_params()?.par_iter().for_each(|x| {
            assert_eq!(
                x.get_dims().unwrap_or(&vec![]),
                inst.function.return_types[0]
                    .dimensions
                    .as_ref()
                    .unwrap_or(&vec![])
            );
        });

        let mut result: Vec<HashMap<&'a str, i8>> = vec![];
        let params = inst.get_all_params()?;
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

    fn d_reshape(inst: &'a Instruction) -> Result<Vec<HashMap<&'a str, i8>>, Box<dyn Error>> {
        inst.assert_param_len(1);
        let all_dims: Vec<i32> = inst.get_param(0)?.get_all_dims_index()?;
        let before_dims = inst.get_param(0)?.get_dims()?;
        let after_dims = inst.function.return_types[0]
            .dimensions
            .as_ref()
            .ok_or(OptionNone("inst.fn.return_type.dimensions".into()))?;
        let (l_dims, r_dims): (&Vec<i32>, &Vec<i32>);
        if before_dims.len() < after_dims.len() {
            l_dims = before_dims;
            r_dims = after_dims;
        } else {
            l_dims = after_dims;
            r_dims = before_dims;
        }
        let mut result: Vec<HashMap<&'a str, i8>> = vec![];
        let mut map_dims: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut rev_map_dims: HashMap<i32, i32> = HashMap::new();
        let param: &'a str = &inst.get_param(0)?.name;
        let var_name: &'a str = &inst.var_name;

        let mut cur = 0usize;
        debug!("setting cur = {}", cur);
        let max_look_ahead = 4usize;
        for (i, x) in l_dims.iter().enumerate() {
            debug!("checking x = {}", *x);
            let mut suc = false;
            for id in (0..max_look_ahead).rev() {
                // let cur_prod: i32 = r_dims[cur..=cur + id].iter().product();
                if r_dims.len() > cur + id {
                    debug!(
                        "comparing x with {}",
                        r_dims[cur..=cur + id].iter().product::<i32>()
                    );
                }
                if r_dims.len() > cur + id
                    && *x as i32 == r_dims[cur..=cur + id].iter().product::<i32>()
                    && ((i != l_dims.len() - 1 && cur + id != r_dims.len() - 1)
                        || (i == l_dims.len() - 1 && cur + id == r_dims.len() - 1))
                // last condition is ad-hoc to (3, 35, 1) -> (3, 35, 1, 1)
                {
                    map_dims.insert(i as i32, (cur..=cur + id).map(|x| x as i32).collect());
                    (cur..=cur + id).for_each(|x| {
                        rev_map_dims.insert(x as i32, i as i32);
                    });
                    cur += id + 1;
                    debug!("setting cur = {}", cur);
                    suc = true;
                    break;
                }
            }
            if !suc {
                return Err(format!(
                    "No product match for inst {} = fn {} ...",
                    inst.var_name, inst.function.name
                )
                .into());
            }
        }

        if before_dims.len() < after_dims.len() {
            for d in all_dims.iter() {
                Self::add_keys(
                    &mut result,
                    vec![(param, *d as i8), (var_name, map_dims[d][0] as i8)],
                )?;
            }
        } else {
            for d in all_dims.iter() {
                if !rev_map_dims.contains_key(d) {
                    print!(
                        "rev_map_dims request key {} cannot be found\ninst {} = fn {} ...",
                        d, inst.var_name, inst.function.name
                    );
                    return Err(format!(
                        "rev_map_dims request key {} cannot be found\ninst {} = fn {} ...",
                        d, inst.var_name, inst.function.name
                    )
                    .into());
                }
                Self::add_keys(
                    &mut result,
                    vec![(param, *d as i8), (var_name, rev_map_dims[d] as i8)],
                )?;
            }
        }

        // add replication
        Self::add_keys(&mut result, Self::replicate_split(inst)?)?;

        Ok(result)
    }

    fn d_reshape_alt(inst: &'a Instruction) -> Result<Vec<HashMap<&'a str, i8>>, Box<dyn Error>> {
        inst.assert_param_len(1);
        let all_dims: Vec<i32> = inst.get_param(0)?.get_all_dims_index()?;
        let params_dims = inst.get_param(0)?.get_dims()?;
        let var_dims = inst.function.return_types[0]
            .dimensions
            .as_ref()
            .ok_or(OptionNone("inst.fn.return_type.dimensions".into()))?;
        let mut p_i = 0;
        let mut v_i = 0;
        let mut l_prod = 1;
        let mut r_prod = 1;
        let mut params_groups: Vec<Vec<i8>> = vec![];
        let mut var_groups: Vec<Vec<i8>> = vec![];
        while (p_i < params_dims.len() && v_i < var_dims.len()) {
            while l_prod < r_prod && v_i < var_dims.len() {
                l_prod *= var_dims[v_i];
                let v_1 = var_groups.len() - 1;
                var_groups[v_1].push(v_i as i8);
                v_i += 1;
            }
            while l_prod > r_prod && p_i < params_dims.len() {
                r_prod *= params_dims[p_i];
                let p_1 = params_groups.len() - 1;
                params_groups[p_1].push(p_i as i8);
                p_i += 1;
            }
            if l_prod == r_prod && v_i < var_dims.len() && p_i < params_dims.len() {
                var_groups.push(vec![]);
                params_groups.push(vec![]);
                l_prod = var_dims[v_i];
                r_prod = params_dims[p_i];
                let v_1 = var_groups.len() - 1;
                var_groups[v_1].push(v_i as i8);
                let p_1 = params_groups.len() - 1;
                params_groups[p_1].push(p_i as i8);
                v_i += 1;
                p_i += 1;
            }
        }

        while p_i < params_dims.len() {
            let p_1 = params_groups.len() - 1;
            params_groups[p_1].push(p_i as i8);
            p_i += 1;
        }
        while v_i < var_dims.len() {
            let v_1 = var_groups.len() - 1;
            var_groups[v_1].push(v_i as i8);
            v_i += 1;
        }
        let mut result: Vec<HashMap<&'a str, i8>> = vec![];
        let param: &'a str = &inst.get_param(0)?.name;
        let var_name: &'a str = &inst.var_name;
        for (i, x) in var_groups.iter().enumerate() {
            for dim_var in x {
                for dim_params in &params_groups[i] {
                    Self::add_keys(
                        &mut result,
                        vec![(param, *dim_params as i8), (var_name, *dim_var as i8)],
                    )?;
                }
            }
        }
        Self::add_keys(&mut result, Self::replicate_split(inst)?)?;
        Ok(result)
    }

    fn d_reduce(inst: &'a Instruction) -> Result<Vec<HashMap<&'a str, i8>>, Box<dyn Error>> {
        inst.assert_param_len(2);
        inst.assert_key_in_meta("dimensions");
        let mut result: Vec<HashMap<&'a str, i8>> = vec![];
        let reduce_dims = inst.get_meta_vec("dimensions")?;
        let var_name: &'a str = &inst.var_name;
        let all_dims: Vec<i32> = inst.get_param(0)?.get_all_dims_index()?;
        let params = inst.get_all_params()?;
        for d in all_dims.iter() {
            if reduce_dims.contains(d) {
                Self::add_keys(
                    &mut result,
                    vec![
                        (var_name, -1i8),
                        (&params[0].name, *d as i8),
                        (&params[1].name, -1),
                    ],
                )?;
            } else {
                let mut split_dim = *d;
                for rd in reduce_dims.iter() {
                    if *rd < *d {
                        split_dim -= 1;
                    }
                }
                Self::add_keys(
                    &mut result,
                    vec![
                        (var_name, split_dim as i8),
                        (&params[0].name, *d as i8),
                        (&params[1].name, -1),
                    ],
                )?;
            }
        }

        Self::add_keys(&mut result, Self::replicate_split(inst)?)?;

        return Ok(result);
    }

    fn d_transpose(inst: &'a Instruction) -> Result<Vec<HashMap<&'a str, i8>>, Box<dyn Error>> {
        inst.assert_param_len(1);
        inst.assert_key_in_meta("dimensions");
        let transpose_dims = inst.get_meta_vec("dimensions")?;
        let all_dims: Vec<i32> = inst.get_param(0)?.get_all_dims_index()?;
        assert_eq!(transpose_dims.len(), all_dims.len());
        let param: &'a str = &inst.get_param(0)?.name;
        let var_name: &'a str = &inst.var_name;
        let mut result: Vec<HashMap<&'a str, i8>> = vec![];
        for d in all_dims.iter() {
            Self::add_keys(
                &mut result,
                vec![
                    (
                        var_name,
                        transpose_dims.iter().position(|x| *x == *d).unwrap() as i8,
                    ),
                    (param, *d as i8),
                ],
            )?;
        }

        Self::add_keys(&mut result, Self::replicate_split(inst)?)?;

        Ok(result)
    }

    fn d_gather(inst: &'a Instruction) -> Result<Vec<HashMap<&'a str, i8>>, Box<dyn Error>> {
        inst.assert_param_len(2);
        inst.assert_key_in_meta("offset_dims");
        assert_eq!(inst.get_param(0)?.get_dims()?.len(), 2);
        let offset_dims = inst.get_meta_vec("offset_dims")?;
        assert_eq!(offset_dims.len(), 1);
        let mut result: Vec<HashMap<&'a str, i8>> = vec![];
        let params = inst.get_all_params()?;
        let var_name: &'a str = &inst.var_name;

        // split at param 0
        {
            Self::add_keys(
                &mut result,
                vec![
                    (&params[0].name, 1i8),
                    (&params[1].name, -1i8),
                    (var_name, offset_dims[0] as i8),
                ],
            )?;
            Self::add_keys(
                &mut result,
                vec![
                    (&params[0].name, 0i8),
                    (&params[1].name, -1i8),
                    (var_name, -1i8),
                ],
            )?;
        }

        // split at param 1
        {
            Self::add_keys(
                &mut result,
                vec![
                    (&params[1].name, 0i8),
                    (&params[0].name, 0i8),
                    (var_name, 0i8),
                ],
            )?;
            Self::add_keys(
                &mut result,
                vec![
                    (&params[1].name, -1i8),
                    (&params[0].name, 1i8),
                    (var_name, 1i8),
                ],
            )?;
        }

        // add replication
        Self::add_keys(&mut result, Self::replicate_split(inst)?)?;

        Ok(result)
    }

    fn d_scatter(inst: &'a Instruction) -> Result<Vec<HashMap<&'a str, i8>>, Box<dyn Error>> {
        inst.assert_param_len(3);
        let params = inst.get_all_params()?;
        let var_name: &'a str = &inst.var_name;
        assert_eq!(params[0].get_dims()?.len(), 2);
        assert_eq!(params[2].get_dims()?.len(), 2);
        let mut result: Vec<HashMap<&'a str, i8>> = vec![];

        // split at param 0
        {
            // 0, 0
            Self::add_keys(
                &mut result,
                vec![
                    (&params[0].name, 0),
                    (&params[1].name, -1),
                    (&params[2].name, -1),
                    (var_name, 0),
                ],
            )?;
            // 0, 1
            Self::add_keys(
                &mut result,
                vec![
                    (&params[0].name, 1),
                    (&params[1].name, -1),
                    (&params[2].name, 1),
                    (var_name, 1),
                ],
            )?;
        }

        // split at param 1, dim 1
        {
            Self::add_keys(
                &mut result,
                vec![
                    (&params[1].name, -1),
                    (&params[0].name, 0),
                    (&params[2].name, 0),
                    (var_name, 0),
                ],
            )?;
        }

        // split at param 2
        {
            // 2, 1
            Self::add_keys(
                &mut result,
                vec![
                    (&params[2].name, 1),
                    (&params[0].name, 1),
                    (&params[1].name, -1),
                    (var_name, 1),
                ],
            )?;
            // 2, -1
            Self::add_keys(
                &mut result,
                vec![
                    (&params[2].name, -1),
                    (&params[0].name, 0),
                    (&params[1].name, -1),
                    (var_name, 0),
                ],
            )?;
        }

        Self::add_keys(&mut result, Self::replicate_split(inst)?)?;

        Ok(result)
    }

    fn d_pad_slice(inst: &'a Instruction) -> Result<Vec<HashMap<&'a str, i8>>, Box<dyn Error>> {
        match inst.function.name.as_str() {
            "pad" => inst.assert_param_len(2),
            "slice" => inst.assert_param_len(1),
            _ => assert_eq!(0, 1, "this fn cannot be derived using pad_slice"),
        }
        let var_name: &'a str = &inst.var_name;
        let params = inst.get_all_params()?;
        let param_dims = params[0].get_dims()?;
        let output_dims = inst.function.return_types[0]
            .dimensions
            .as_ref()
            .ok_or(OptionNone("inst.fn.return_type.dimensions".into()))?;
        assert_eq!(param_dims.len(), output_dims.len());

        let mut result: Vec<HashMap<&'a str, i8>> = vec![];

        let diff_index: Vec<i32> = param_dims
            .iter()
            .enumerate()
            .filter_map(|(i, _x)| {
                if param_dims[i] != output_dims[i] {
                    Some(i as i32)
                } else {
                    None
                }
            })
            .collect();

        for d in 0..output_dims.len() as i32 {
            if !diff_index.contains(&d) {
                let mut splits: Vec<(&'a str, i8)> =
                    vec![(&params[0].name, d as i8), (var_name, d as i8)];
                if params.len() == 2 {
                    splits.push((&params[1].name, -1));
                }
                Self::add_keys(&mut result, splits)?;
            }
        }

        Self::add_keys(&mut result, Self::replicate_split(inst)?)?;

        Ok(result)
    }

    fn d_concat(inst: &'a Instruction) -> Result<Vec<HashMap<&'a str, i8>>, Box<dyn Error>> {
        // inst.assert_param_len(2);
        inst.assert_key_in_meta("dimensions");
        let mut result: Vec<HashMap<&'a str, i8>> = vec![];
        let concat_dims = inst.get_meta_vec("dimensions")?;
        let all_dims = inst.get_param(0)?.get_all_dims_index()?;
        let params = inst.get_all_params()?;
        let var_name: &'a str = &inst.var_name;
        for d in all_dims.iter() {
            if !concat_dims.contains(d) {
                let mut splits: Vec<(&'a str, i8)> = vec![(var_name, *d as i8)];
                for p in params {
                    splits.push((&p.name, *d as i8));
                }
                Self::add_keys(&mut result, splits)?;
            }
        }
        Self::add_keys(&mut result, Self::replicate_split(inst)?)?;

        Ok(result)
    }

    fn d_padded_where(inst: &'a Instruction) -> Result<Vec<HashMap<&'a str, i8>>, Box<dyn Error>> {
        inst.assert_param_len(1);

        // NOTE: ad-hoc impl
        assert_eq!(inst.get_param(0)?.get_dims()?.len(), 1);
        assert_eq!(
            inst.function.return_types[0]
                .dimensions
                .as_ref()
                .unwrap_or(&vec![])
                .len(),
            2
        );
        let var_name: &'a str = &inst.var_name;
        let mut result: Vec<HashMap<&'a str, i8>> = vec![];

        // first param split by 0
        {
            Self::add_keys(
                &mut result,
                vec![(var_name, 0), (&inst.get_param(0)?.name, 0)],
            )?;
        }

        Self::add_keys(&mut result, Self::replicate_split(inst)?)?;

        Ok(result)
    }

    fn d_broadcast(inst: &'a Instruction) -> Result<Vec<HashMap<&'a str, i8>>, Box<dyn Error>> {
        inst.assert_param_len(1);
        inst.assert_key_in_meta("dimensions");
        let mut result: Vec<HashMap<&'a str, i8>> = vec![];
        let empty_placeholder: Vec<i32> = vec![];
        let standby_dims = inst
            .get_meta_vec("dimensions")
            .unwrap_or(&empty_placeholder);

        let output_dims = inst.function.return_types[0]
            .dimensions
            .as_ref()
            .ok_or(OptionNone("inst.fn.return_type.dimensions".into()))?;
        assert_eq!(
            inst.get_param(0)?
                .get_dims()
                .unwrap_or(&empty_placeholder)
                .len(),
            standby_dims.len()
        );
        let var_name: &'a str = &inst.var_name;
        let all_dims: Vec<i32> = (0..output_dims.len() as i32).collect();
        for d in all_dims.iter() {
            if standby_dims.contains(d) {
                let index_at_param = standby_dims.iter().position(|x| *x == *d).unwrap();
                Self::add_keys(
                    &mut result,
                    vec![
                        (var_name, *d as i8),
                        (&inst.get_param(0)?.name, index_at_param as i8),
                    ],
                )?;
            } else {
                Self::add_keys(
                    &mut result,
                    vec![(var_name, *d as i8), (&inst.get_param(0)?.name, -1i8)],
                )?;
            }
        }

        Self::add_keys(&mut result, Self::replicate_split(inst)?)?;

        Ok(result)
    }

    fn replicate_split(inst: &'a Instruction) -> Result<Vec<(&'a str, i8)>, Box<dyn Error>> {
        let params = inst.get_all_params()?;
        let var_name: &'a str = &inst.var_name;

        // iterate over all dimensions index, including -1 for replication
        let mut splits: Vec<(&'a str, i8)> = vec![(var_name, -1i8)];
        for p in params {
            splits.push((&p.name, -1i8))
        }
        Ok(splits)
    }

    fn d_replicate(inst: &'a Instruction) -> Result<Vec<HashMap<&'a str, i8>>, Box<dyn Error>> {
        let mut result: Vec<HashMap<&'a str, i8>> = vec![];
        Self::add_keys(&mut result, Self::replicate_split(inst)?)?;
        Ok(result)
    }

    fn d_identity(inst: &'a Instruction) -> Result<Vec<HashMap<&'a str, i8>>, Box<dyn Error>> {
        let mut result: Vec<HashMap<&'a str, i8>> = vec![];
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
            let splits: Vec<(&'a str, i8)> = vec![(var_name, d as i8)];
            Self::add_keys(&mut result, splits)?;
        }

        Ok(result)
    }
}
