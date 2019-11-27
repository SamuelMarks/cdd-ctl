use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Variable {
    pub name: String,
    #[serde(rename = "type")]
    pub variable_type: VariableType,
    pub optional: bool,
    pub value: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
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

impl std::fmt::Display for VariableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "yes")
    }
}

impl serde::Serialize for VariableType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        let s = match &self {
            VariableType::StringType => "String",
            VariableType::IntType => "Int",
            VariableType::BoolType => "Bool",
            VariableType::FloatType => "Float",
            VariableType::ArrayType(ref vartype) => {
                // recursive enums are the devil, we should be using reference counting here
                unsafe {
                    match *Box::into_raw(vartype.clone()) {
                        VariableType::StringType => "[String]",
                        VariableType::IntType => "[Int]",
                        VariableType::BoolType => "[Bool]",
                        VariableType::FloatType => "[Float]",
                        _ => "[Complex]",
                    }
                }
            },
            VariableType::ComplexType(ref vartype) => {
                &(*vartype)
            },
        };
        
        serializer.serialize_str(&s)
    }
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

#[test]
fn test_json() {
    assert_eq!(serde_json::to_string(&VariableType::StringType).unwrap(), "\"String\"");
    assert_eq!(serde_json::to_string(&VariableType::ArrayType(Box::new(VariableType::StringType))).unwrap(), "[\"String\"]");
}
