use crate::ir::error::ASTError;
use crate::ir::error::DeriveError::*;
use log::debug;
use pyo3::prelude::*;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

const REF: &str = "https://ry.sb/tf/xla-op";
// NOTE: did not use HashSet here because PyO3 does not impl IntoPyResult
pub type VarPos = (usize, Vec<usize>);
pub type InstPos = (usize, usize);

#[pyclass]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Default)]
#[serde(default)]
pub struct HLORoot {
    #[pyo3(get)]
    #[serde(rename(deserialize = "Functions"))]
    pub functions: Vec<HLOFunction>,

    // relying on the fact that Function Names are unique
    #[pyo3(get)]
    pub func_id: HashMap<String, usize>,
    #[pyo3(get)]
    pub inst_id: HashMap<Instruction, usize>,
    #[pyo3(get)]
    pub inst_local_id: HashMap<Instruction, usize>,
    #[pyo3(get)]
    pub inst_pos: HashMap<Instruction, InstPos>,
    // relying on the fact that Variable Names are unique
    #[pyo3(get)]
    pub var_pos: HashMap<String, VarPos>,
}

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct HLOFunction {
    #[pyo3(get)]
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[pyo3(get)]
    #[serde(rename(deserialize = "Params"))]
    pub params: Vec<Param>,
    #[pyo3(get)]
    #[serde(rename(deserialize = "ReturnTypes"))]
    pub return_type: Type,
    #[pyo3(get)]
    #[serde(rename(deserialize = "Body"))]
    pub body: Vec<Instruction>,
}

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Instruction {
    #[pyo3(get)]
    #[serde(rename(deserialize = "VarName"))]
    pub var_name: String,
    #[pyo3(get)]
    #[serde(rename(deserialize = "Fn"))]
    pub function: FunctionCall,
    #[pyo3(get)]
    #[serde(rename(deserialize = "Meta"))]
    pub meta: Option<Vec<Meta>>,
}

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct FunctionCall {
    #[pyo3(get)]
    #[serde(rename(deserialize = "ReturnType"))]
    pub return_type: Type,
    #[pyo3(get)]
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[pyo3(get)]
    #[serde(rename(deserialize = "Argument"))]
    pub args: Option<Vec<Argument>>,
}

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Meta {
    #[pyo3(get)]
    #[serde(rename(deserialize = "Key"))]
    pub key: String,
    #[pyo3(get)]
    #[serde(rename(deserialize = "Value"))]
    pub value: Value,
    // pub str_value: Option<String>,
    // #[pyo3(get)]
    // #[serde(rename(deserialize = "DictValue"))]
    // pub dict_value: Option<Vec<Dict>>,
    // #[pyo3(get)]
    // #[serde(rename(deserialize = "ListNums"))]
    // pub num_list: Option<Vec<i32>>,
    // #[pyo3(get)]
    // #[serde(rename(deserialize = "ListSlices"))]
    // pub slice_list: Option<Vec<Slice>>,
}

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Value {
    #[pyo3(get)]
    #[serde(rename(deserialize = "Number"))]
    pub number: Option<i32>,
    #[pyo3(get)]
    #[serde(rename(deserialize = "String"))]
    pub string: Option<String>,
    #[pyo3(get)]
    #[serde(rename(deserialize = "Numbers"))]
    pub numbers: Option<Vec<i32>>,
    #[pyo3(get)]
    #[serde(rename(deserialize = "Dicts"))]
    pub dicts: Option<Vec<Dict>>,
    #[pyo3(get)]
    #[serde(rename(deserialize = "Slices"))]
    pub slices: Option<Vec<Slice>>,
    #[pyo3(get)]
    #[serde(rename(deserialize = "Boolean"))]
    pub boolean: Option<bool>,
    #[pyo3(get)]
    #[serde(rename(deserialize = "Misc"))]
    pub misc: Option<String>,
}

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Dict {
    #[pyo3(get)]
    #[serde(rename(deserialize = "Key"))]
    pub key: String,
    #[pyo3(get)]
    #[serde(rename(deserialize = "Value"))]
    pub value: Value,
}

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Slice {
    #[pyo3(get)]
    #[serde(rename(deserialize = "Start"))]
    pub start: i32,
    #[pyo3(get)]
    #[serde(rename(deserialize = "End"))]
    pub end: i32,
}

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Param {
    #[pyo3(get)]
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[pyo3(get)]
    #[serde(rename(deserialize = "Type"))]
    pub param_type: Type,
}

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Argument {
    #[pyo3(get)]
    #[serde(rename(deserialize = "Type"))]
    pub param_type: Type,
    #[pyo3(get)]
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
}

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Type {
    #[pyo3(get)]
    #[serde(rename(deserialize = "DataType"))]
    pub data_type: String,
    #[pyo3(get)]
    #[serde(rename(deserialize = "Dimensions"))]
    pub dimensions: Option<Vec<i32>>,
    #[pyo3(get)]
    #[serde(rename(deserialize = "Layout"))]
    pub layout: Option<Vec<i32>>,
    #[pyo3(get)]
    #[serde(rename(deserialize = "TupleType"))]
    pub typle_type: Option<Vec<Type>>,
}

impl HLORoot {
    pub fn cache_positional_func(&mut self) -> Result<(), Box<dyn Error>> {
        if self.func_id.len() != 0 {
            return Err(Box::new(ASTError::CacheFuncTwice));
        }
        self.func_id.extend(
            self.functions
                .iter()
                .enumerate()
                .map(|(i, f)| (f.name.clone(), i)),
        );
        assert_eq!(self.func_id.len() > 0, true);
        Ok(())
    }

    pub fn cache_positional_inst(&mut self) -> Result<(), Box<dyn Error>> {
        if self.inst_id.len() != 0 {
            return Err(Box::new(ASTError::CacheInstTwice));
        }
        // workaround to increase index before returning
        let mut index = -1;
        let f_pos = self
            .functions
            .iter()
            .enumerate()
            .flat_map(|(ind_f, f)| {
                // let mut local_index = -1;
                f.body
                    .iter()
                    .enumerate()
                    .map(|(ind_i, i)| {
                        index += 1;
                        // local_index += 1;
                        (i.clone(), index as usize, ind_f, ind_i)
                    })
                    .collect::<Vec<(Instruction, usize, usize, usize)>>()
            })
            .collect::<Vec<(Instruction, usize, usize, usize)>>();
        self.inst_id.par_extend(
            f_pos
                .par_iter()
                .map(|(inst, index, _, _)| (inst.clone(), *index)),
        );
        self.inst_local_id.par_extend(
            f_pos
                .par_iter()
                .map(|(inst, _, _, local_index)| (inst.clone(), *local_index)),
        );
        self.inst_pos.par_extend(
            f_pos
                .par_iter()
                .map(|(inst, _, ind_f, ind_i)| (inst.clone(), (*ind_f, *ind_i))),
        );
        assert_eq!(self.inst_id.len() > 0, true);
        Ok(())
    }

    pub fn cache_var_position(&mut self) -> Result<(), Box<dyn Error>> {
        if self.var_pos.len() != 0 {
            return Err(Box::new(ASTError::CacheVarPosTwice));
        }
        let mut var_map: HashMap<String, VarPos> = HashMap::new();
        fn add_to_map(
            var_map: &mut HashMap<String, VarPos>,
            var_name: String,
            func_id: usize,
            inst_local_id: usize,
        ) {
            if !var_map.contains_key(&var_name) {
                var_map.insert(
                    var_name,
                    (func_id, [inst_local_id].iter().cloned().collect()),
                );
            } else {
                assert_eq!(var_map[&var_name].0, func_id);
                if !var_map[&var_name].1.contains(&inst_local_id) {
                    let mut cur_ids = var_map[&var_name].1.clone();
                    cur_ids.push(inst_local_id);
                    *var_map.get_mut(&var_name).unwrap() = (func_id, cur_ids);
                }
            }
        }
        for i in 0..self.functions.len() {
            debug!("caching function {}: {}", i, self.functions[i].name);
            for j in 0..self.functions[i].body.len() {
                let inst = &self.functions[i].body[j];
                add_to_map(&mut var_map, inst.var_name.clone(), i, j);
                let all_params = inst.get_all_params();
                if all_params.is_err() {
                    continue;
                }
                for p in all_params?.iter() {
                    if !p.name.contains("%") {
                        continue;
                    }
                    add_to_map(&mut var_map, p.name.clone(), i, j);
                }
            }
        }
        self.var_pos = var_map;
        Ok(())
    }

    pub fn cache_all_positional(&mut self) -> Result<(), Box<dyn Error>> {
        let now = Instant::now();
        self.cache_positional_func()?;
        self.cache_positional_inst()?;
        self.cache_var_position()?;
        println!(
            "[cache]\t AST Positional Cache {}ms",
            now.elapsed().as_millis()
        );
        Ok(())
    }
}

impl Instruction {
    #[inline]
    pub fn assert_param_len(&self, l: usize) {
        assert_eq!(
            self.function.args.as_ref().unwrap().len(),
            l,
            "{} does not take more than 2 operands, ref: {}",
            self.function.name,
            REF
        );
    }

    #[inline]
    pub fn key_in_meta(&self, s: &str) -> bool {
        self.meta.as_ref().unwrap().par_iter().any(|x| x.key == s)
    }

    #[inline]
    pub fn assert_key_in_meta(&self, s: &str) {
        assert_eq!(self.key_in_meta(s), true)
    }

    #[inline]
    pub fn get_all_params(&self) -> Result<&Vec<Argument>, Box<dyn Error>> {
        let params = self
            .function
            .args
            .as_ref()
            .ok_or(OptionNone("inst.fn.params".into()))?;
        Ok(&params)
    }

    #[inline]
    pub fn get_param(&self, i: usize) -> Result<&Argument, Box<dyn Error>> {
        let params = self
            .function
            .args
            .as_ref()
            .ok_or(OptionNone("inst.fn.params".into()))?;
        Ok(&params[i])
    }

    /// get the return dims of a function call
    pub fn get_return_dims(&self) -> Result<&Vec<i32>, Box<dyn Error>> {
        let dims: &Vec<i32> = self
            .function
            .return_type
            .dimensions
            .as_ref()
            .ok_or(OptionNone("inst.return_type.dims".into()))?;
        Ok(dims)
    }

    /// get the return type dimension list, from 0 to len(return_type.dimensions)
    pub fn get_return_dims_index(&self) -> Result<Vec<i32>, Box<dyn Error>> {
        let all_dims: Vec<i32> =
            (0..self.get_return_dims().unwrap_or(&vec![]).len() as i32).collect();
        Ok(all_dims)
    }

    pub fn get_meta_vec(&self, key: &str) -> Result<&Vec<i32>, Box<dyn Error>> {
        Ok(self
            .meta
            .as_ref()
            .ok_or(OptionNone("inst.meta".into()))?
            .par_iter()
            .find_any(|x| x.key == key)
            .ok_or(MetaKeyNotFound(key.into()))?
            .value
            .numbers
            .as_ref()
            .ok_or(MetaValueNotFound("num_list".into()))?)
    }

    pub fn get_meta_str(&self, key: &str) -> Result<String, Box<dyn Error>> {
        Ok(self
            .meta
            .as_ref()
            .ok_or(OptionNone("inst.meta".into()))?
            .par_iter()
            .find_any(|x| x.key == key)
            .ok_or(MetaKeyNotFound(key.into()))?
            .value
            .string
            .clone()
            .ok_or(MetaValueNotFound("str_value".into()))?)
    }
}

impl Argument {
    pub fn get_all_dims_index(&self) -> Result<Vec<i32>, Box<dyn Error>> {
        let all_dims: Vec<i32> = (0..self.get_dims().unwrap_or(&vec![]).len() as i32).collect();
        Ok(all_dims)
    }

    pub fn get_dims(&self) -> Result<&Vec<i32>, Box<dyn Error>> {
        let dims: &Vec<i32> = self
            .param_type
            .dimensions
            .as_ref()
            .ok_or(OptionNone("rich_param.param_type.dimensions".into()))?;
        Ok(dims)
    }
}

impl Param {
    pub fn get_all_dims_index(&self) -> Result<Vec<i32>, Box<dyn Error>> {
        let all_dims: Vec<i32> = (0..self.get_dims().unwrap_or(&vec![]).len() as i32).collect();
        Ok(all_dims)
    }

    pub fn get_dims(&self) -> Result<&Vec<i32>, Box<dyn Error>> {
        let dims: &Vec<i32> = self
            .param_type
            .dimensions
            .as_ref()
            .ok_or(OptionNone("rich_param.param_type.dimensions".into()))?;
        Ok(dims)
    }
}

impl Param {
    pub fn augment_name(&mut self) {
        if self.name.contains("%") {
            return;
        }
        self.name = format!("%{}", self.name);
    }
}
