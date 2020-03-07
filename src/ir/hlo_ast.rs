pub struct HLORoot {
    pub functions: Vec<HLOFunction>,
}

pub struct HLOFunction {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Type,
    pub body: Vec<Instruction>,
}

pub struct Instruction {
    pub var_name: String,
    pub function: FunctionCall,
    pub meta: Vec<Meta>,
}

pub struct FunctionCall {
    pub return_type: RichType,
    pub name: String,
    pub params: Vec<RichParam>
}

pub struct Meta {
    pub key: String,
    pub str_value: Option<String>,
    pub dict_value: Option<Vec<Dict>>,
    pub num_list: Option<Vec<i32>>,
    pub slice_list: Option<Vec<Slice>>,
}

pub struct Dict {
    pub key: String,
    pub value: String,
}

pub struct Slice {
    pub start: i32,
    pub end: i32,
}

pub struct Param {
    pub name: String,
    pub param_type: Type,
}

pub struct Type {
    pub data_type: String,
    pub dimensions: Vec<i32>,
}

pub struct RichParam {
    pub param_type: RichType,
    pub name: String,
}

pub struct RichType {
    pub data_type: String,
    pub dimensions: Vec<i32>,
    pub layout: Vec<i32>,
}
