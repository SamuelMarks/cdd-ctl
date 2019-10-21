use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Request {
    pub name: String,
    pub path: String,
    pub vars: Vec<Box<Variable>>,
    pub method: Method,
    pub response_type: String,
    pub error_type: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum Method {
    #[serde(rename = "GET")]
    Get_,
    #[serde(rename = "POST")]
    Post_,
    #[serde(rename = "PUT")]
    Put_,
    #[serde(rename = "DELETE")]
    Delete_,
    #[serde(rename = "OPTIONS")]
    Options_,
    #[serde(rename = "HEAD")]
    Head_,
    #[serde(rename = "PATCH")]
    Patch_,
    #[serde(rename = "TRACE")]
    Trace_,

}
impl Method {
    pub fn string(self) -> String {
        match self {
            Get_ => "GET".to_string(),
            Post_ => "POST".to_string(),
            Put_ => "PUT".to_string(),
            Delete_ => "DELETE".to_string(),
            Options_ => "OPTIONS".to_string(),
            Head_ => "HEAD".to_string(),
            Patch_ => "PATCH".to_string(),
            Trace_ => "TRACE".to_string()
        }
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}