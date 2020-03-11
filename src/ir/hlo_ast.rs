use crate::ir::error::DeriveError::*;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;

const REF: &str = "https://ry.sb/tf/xla-op";

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct HLORoot {
    #[serde(rename(deserialize = "Functions"))]
    pub functions: Vec<HLOFunction>,
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct HLOFunction {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Params"))]
    pub params: Vec<Param>,
    #[serde(rename(deserialize = "ReturnTypes"))]
    pub return_types: Vec<Type>,
    #[serde(rename(deserialize = "Body"))]
    pub body: Vec<Instruction>,
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct Instruction {
    #[serde(rename(deserialize = "VarName"))]
    pub var_name: String,
    #[serde(rename(deserialize = "Fn"))]
    pub function: FunctionCall,
    #[serde(rename(deserialize = "Meta"))]
    pub meta: Option<Vec<Meta>>,
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct FunctionCall {
    #[serde(rename(deserialize = "ReturnTypes"))]
    pub return_types: Vec<RichType>,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Params"))]
    pub params: Option<Vec<RichParam>>,
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct Meta {
    #[serde(rename(deserialize = "Key"))]
    pub key: String,
    #[serde(rename(deserialize = "Value"))]
    pub str_value: Option<String>,
    #[serde(rename(deserialize = "DictValue"))]
    pub dict_value: Option<Vec<Dict>>,
    #[serde(rename(deserialize = "ListNums"))]
    pub num_list: Option<Vec<i32>>,
    #[serde(rename(deserialize = "ListSlices"))]
    pub slice_list: Option<Vec<Slice>>,
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct Dict {
    #[serde(rename(deserialize = "Key"))]
    pub key: String,
    #[serde(rename(deserialize = "Value"))]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct Slice {
    #[serde(rename(deserialize = "Start"))]
    pub start: i32,
    #[serde(rename(deserialize = "End"))]
    pub end: i32,
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct Param {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Type"))]
    pub param_type: Type,
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct Type {
    #[serde(rename(deserialize = "DataType"))]
    pub data_type: String,
    #[serde(rename(deserialize = "Dimensions"))]
    pub dimensions: Option<Vec<i32>>,
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct RichParam {
    #[serde(rename(deserialize = "Type"))]
    pub param_type: RichType,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct RichType {
    #[serde(rename(deserialize = "DataType"))]
    pub data_type: String,
    #[serde(rename(deserialize = "Dimensions"))]
    pub dimensions: Option<Vec<i32>>,
    #[serde(rename(deserialize = "Layout"))]
    pub layout: Option<Vec<i32>>,
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
    pub fn get_meta_vec(&self, key: &str) -> Result<&Vec<i32>, Box<dyn Error>> {
        Ok(self
            .meta
            .as_ref()
            .ok_or(OptionNone("inst.meta".into()))?
            .par_iter()
            .find(|x| x.key == key)
            .ok_or(MetaKeyNotFound(key.into()))?
            .num_list
            .as_ref()
            .ok_or(MetaValueNotFound("num_list".into()))?)
    }
}
