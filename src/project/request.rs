use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Request {
    pub name: String,
    pub fields: Vec<Box<Variable>>,
    pub method: Method,
    pub response_type: String,
    pub error_type: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum Method {
    #[serde(rename(serialize = "GET"))]
    Get_,
    #[serde(rename(serialize = "POST"))]
    Post_,
    #[serde(rename(serialize = "PUT"))]
    Put_,
    #[serde(rename(serialize = "DELETE"))]
    Delete_,
    #[serde(rename(serialize = "OPTIONS"))]
    Options_,
    #[serde(rename(serialize = "HEAD"))]
    Head_,
    #[serde(rename(serialize = "PATCH"))]
    Patch_,
    #[serde(rename(serialize = "TRACE"))]
    Trace_,
}
