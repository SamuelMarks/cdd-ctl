use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Variable {
    pub name: String,
    pub variable_type: VariableType,
    pub optional: bool,
    pub value: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum VariableType {
    StringType,
    IntType,
    BoolType,
    FloatType,
    ArrayType(Box<VariableType>),
    ComplexType(String),
}
