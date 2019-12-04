use crate::config::Config;
use crate::project::*;
use crate::*;
use log::*;
use openapiv3::OpenAPI;
use std::path::PathBuf;

pub struct ProjectGraph {
    config: Config,
    spec: OpenAPI,
}

impl ProjectGraph {
    pub fn read(path: &PathBuf) -> CliResult<Self> {
        let config = config::Config::read(path.join("config.yml"))?;
        let spec = load_openapi_spec()?;
        Ok(ProjectGraph {
            config,
            spec,
        })
    }

    /// super basic one way spec -> projects sync
    pub fn simple_sync(&self) -> CliResult<()> {
        let spec_graph = Project::parse_yml(self.spec.clone())?;

        info!(
            "Found {} models, {} routes in {}",
            spec_graph.models.len(),
            spec_graph.requests.len(),
            "openapi.yml"
        );
        for (_name, service) in self.config.services.clone() {
            service.sync_with(&spec_graph)?;
            service.write_tests()?;
        }

        util::write_file(
            PathBuf::from("schema.sql"),
            &schema::generate(spec_graph),
        )?;

        Ok(())
    }

    pub fn copy_templates(&self) -> CliResult<()> {
        info!("Checking project directories");
        for (name, service) in self.config.services.clone() {
            let project_path = service.project_path.clone();
            if !util::file_exists(project_path.clone()) {
                warn!(
                    "Could not find local project for {} at {} - copying fresh template from {}",
                    name.clone(),
                    project_path,
                    service.template_path,
                );

                let template_path = util::expand_home_path(service.template_path.clone())?;
                util::copy_dir(template_path, ".")?;
            } else {
                info!("Found service: {}", name);
            }
        }
        Ok(())
    }
}

fn load_openapi_spec() -> CliResult<OpenAPI> {
    let spec_path: PathBuf = PathBuf::from("openapi.yml");

    if !spec_path.exists() {
        return Err(failure::format_err!("Could not find openapi.yml"));
    };

    let spec = std::fs::read_to_string(spec_path)?;

    Ok(serde_yaml::from_str(&spec)?)
}
