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
    Get_,
    Post_,
    Put_,
    Delete_,
    Options_,
    Head_,
    Patch_,
    Trace_,
}
