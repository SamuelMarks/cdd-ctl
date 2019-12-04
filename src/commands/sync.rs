use crate::*;
use std::path::PathBuf;

pub fn sync() -> CliResult<()> {
    let project_path = PathBuf::from(".");
    let project = project_graph::ProjectGraph::read(&project_path)?;

    // ensure all projects are in place
    project.copy_templates()?;
    project.simple_sync()?;

    Ok(())
}
