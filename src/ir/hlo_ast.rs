use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HLORoot {
    #[serde(rename(deserialize = "Functions"))]
    pub functions: Vec<HLOFunction>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Instruction {
    #[serde(rename(deserialize = "VarName"))]
    pub var_name: String,
    #[serde(rename(deserialize = "Fn"))]
    pub function: FunctionCall,
    #[serde(rename(deserialize = "Meta"))]
    pub meta: Option<Vec<Meta>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FunctionCall {
    #[serde(rename(deserialize = "ReturnTypes"))]
    pub return_types: Vec<RichType>,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Params"))]
    pub params: Option<Vec<RichParam>>
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Dict {
    #[serde(rename(deserialize = "Key"))]
    pub key: String,
    #[serde(rename(deserialize = "Value"))]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Slice {
    #[serde(rename(deserialize = "Start"))]
    pub start: i32,
    #[serde(rename(deserialize = "End"))]
    pub end: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Param {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Type"))]
    pub param_type: Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Type {
    #[serde(rename(deserialize = "DataType"))]
    pub data_type: String,
    #[serde(rename(deserialize = "Dimensions"))]
    pub dimensions: Option<Vec<i32>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RichParam {
    #[serde(rename(deserialize = "Type"))]
    pub param_type: RichType,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RichType {
    #[serde(rename(deserialize = "DataType"))]
    pub data_type: String,
    #[serde(rename(deserialize = "Dimensions"))]
    pub dimensions: Option<Vec<i32>>,
    #[serde(rename(deserialize = "Layout"))]
    pub layout: Option<Vec<i32>>,
}
