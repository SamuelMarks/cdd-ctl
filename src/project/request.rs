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
    pub fn as_string(self) -> String {
        format!("{}", self)
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Get_ => write!(f, "GET"),
            Post_ => write!(f, "POST"),
            Put_ => write!(f, "PUT"),
            Delete_ => write!(f, "DELETE"),
            Options_ => write!(f, "OPTIONS"),
            Head_ => write!(f, "HEAD"),
            Patch_ => write!(f, "PATCH"),
            Trace_ => write!(f, "TRACE"),
        }
    }
}
