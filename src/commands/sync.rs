use crate::*;
use std::path::PathBuf;

pub fn sync() -> CliResult<()> {
    let project_path = PathBuf::from(".");
    let project_graph = project_graph::ProjectGraph::read(&project_path)?;

    // ensure all projects are in place
    project_graph.copy_templates()?;
    project_graph.simple_sync()?;

    Ok(())
}
