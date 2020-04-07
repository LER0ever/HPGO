use crate::ir::error::ASTError;
use crate::ir::error::DeriveError::*;
use pyo3::prelude::*;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;

const REF: &str = "https://ry.sb/tf/xla-op";
// MOTE: did not use HashSet here because PyO3 does not impl IntoPyResult
pub type VarPos = (usize, Vec<usize>);

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
    pub return_types: Vec<Type>,
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
    #[serde(rename(deserialize = "ReturnTypes"))]
    pub return_types: Vec<RichType>,
    #[pyo3(get)]
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[pyo3(get)]
    #[serde(rename(deserialize = "Params"))]
    pub params: Option<Vec<RichParam>>,
}

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Meta {
    #[pyo3(get)]
    #[serde(rename(deserialize = "Key"))]
    pub key: String,
    #[pyo3(get)]
    #[serde(rename(deserialize = "Value"))]
    pub str_value: Option<String>,
    #[pyo3(get)]
    #[serde(rename(deserialize = "DictValue"))]
    pub dict_value: Option<Vec<Dict>>,
    #[pyo3(get)]
    #[serde(rename(deserialize = "ListNums"))]
    pub num_list: Option<Vec<i32>>,
    #[pyo3(get)]
    #[serde(rename(deserialize = "ListSlices"))]
    pub slice_list: Option<Vec<Slice>>,
}

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Dict {
    #[pyo3(get)]
    #[serde(rename(deserialize = "Key"))]
    pub key: String,
    #[pyo3(get)]
    #[serde(rename(deserialize = "Value"))]
    pub value: String,
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
pub struct Type {
    #[pyo3(get)]
    #[serde(rename(deserialize = "DataType"))]
    pub data_type: String,
    #[pyo3(get)]
    #[serde(rename(deserialize = "Dimensions"))]
    pub dimensions: Option<Vec<i32>>,
}

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct RichParam {
    #[pyo3(get)]
    #[serde(rename(deserialize = "Type"))]
    pub param_type: RichType,
    #[pyo3(get)]
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
}

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct RichType {
    #[pyo3(get)]
    #[serde(rename(deserialize = "DataType"))]
    pub data_type: String,
    #[pyo3(get)]
    #[serde(rename(deserialize = "Dimensions"))]
    pub dimensions: Option<Vec<i32>>,
    #[pyo3(get)]
    #[serde(rename(deserialize = "Layout"))]
    pub layout: Option<Vec<i32>>,
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
            .flat_map(|f| {
                let mut local_index = -1;
                f.body
                    .iter()
                    .map(|i| {
                        index += 1;
                        local_index += 1;
                        (i.clone(), index as usize, local_index as usize)
                    })
                    .collect::<Vec<(Instruction, usize, usize)>>()
            })
            .collect::<Vec<(Instruction, usize, usize)>>();
        self.inst_id.par_extend(
            f_pos
                .par_iter()
                .map(|(inst, index, _)| (inst.clone(), *index)),
        );
        self.inst_local_id.par_extend(
            f_pos
                .par_iter()
                .map(|(inst, _, local_index)| (inst.clone(), *local_index)),
        );
        assert_eq!(self.inst_id.len() > 0, true);
        Ok(())
    }

    pub fn cache_var_position(&mut self) -> Result<(), Box<dyn Error>> {
        if self.var_pos.len() != 0 {
            return Err(Box::new(ASTError::CacheVarPosTwice));
        }
        // self.functions.iter().flat_map(|f| {
        //     f.body.iter().map(|i| {
        //         (i.var_name.clone(), self.inst_id[i])
        //     }).collect()
        // });
        unimplemented!()
    }
}

impl Instruction {
    #[inline]
    pub fn assert_param_len(&self, l: usize) {
        assert_eq!(
            self.function.params.as_ref().unwrap().len(),
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
    pub fn get_all_params(&self) -> Result<&Vec<RichParam>, Box<dyn Error>> {
        let params = self
            .function
            .params
            .as_ref()
            .ok_or(OptionNone("inst.fn.params".into()))?;
        Ok(&params)
    }

    #[inline]
    pub fn get_param(&self, i: usize) -> Result<&RichParam, Box<dyn Error>> {
        let params = self
            .function
            .params
            .as_ref()
            .ok_or(OptionNone("inst.fn.params".into()))?;
        Ok(&params[i])
    }

    pub fn get_meta_vec(&self, key: &str) -> Result<&Vec<i32>, Box<dyn Error>> {
        Ok(self
            .meta
            .as_ref()
            .ok_or(OptionNone("inst.meta".into()))?
            .par_iter()
            .find_any(|x| x.key == key)
            .ok_or(MetaKeyNotFound(key.into()))?
            .num_list
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
            .str_value
            .clone()
            .ok_or(MetaValueNotFound("str_value".into()))?)
    }
}

impl RichParam {
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
