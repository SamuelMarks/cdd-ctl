use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Request {
    pub name: String,
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
