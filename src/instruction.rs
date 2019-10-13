use crate::project_graph::*;

#[derive(Debug)]
pub enum Instruction {
    AddModel(Model),
    RemoveModel(Model),
}
