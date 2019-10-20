use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Variable {
    pub name: String,
    #[serde(rename(serialize = "type"))]
    pub variable_type: VariableType,
    pub optional: bool,
    pub value: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum VariableType {
    #[serde(rename(serialize = "String"))]
    StringType,
    #[serde(rename(serialize = "Int"))]
    IntType,
    #[serde(rename(serialize = "Bool"))]
    BoolType,
    #[serde(rename(serialize = "Float"))]
    FloatType,
    #[serde(rename(serialize = "Array"))]
    ArrayType(Box<VariableType>),
    #[serde(rename(serialize = "Complex"))]
    ComplexType(String),
}
