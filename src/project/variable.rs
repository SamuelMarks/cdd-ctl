use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Variable {
    pub name: String,
    #[serde(rename = "type")]
    pub variable_type: VariableType,
    pub optional: bool,
    pub value: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum VariableType {
    #[serde(rename = "String")]
    StringType,
    #[serde(rename = "Int")]
    IntType,
    #[serde(rename = "Bool")]
    BoolType,
    #[serde(rename = "Float")]
    FloatType,
    #[serde(rename = "Array")]
    ArrayType(Box<VariableType>),
    #[serde(rename = "Complex")]
    ComplexType(String),
}
