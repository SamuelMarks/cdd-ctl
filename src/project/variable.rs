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

impl VariableType {
    pub fn to_mysql(&self) -> String {
        match self {
            VariableType::StringType => "TEXT",
            VariableType::IntType => "INT",
            VariableType::BoolType => "TINYINT",
            VariableType::FloatType => "INT",
            _ => "TEXT",
        }
        .to_string()
    }
}
