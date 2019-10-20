use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Model {
    pub name: String,
    pub vars: Vec<Box<Variable>>,
}
