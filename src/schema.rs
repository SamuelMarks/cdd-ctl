use crate::*;
use log::*;

// java -jar openapi-generator-cli.jar generate -i ~/Workspaces/cdd.workspace/testbed/openapi.yml  --generator-name mysql-schema

pub fn generate(path: &str, project: Project) -> String {
    info!("Generating sql migration schema");
    project
        .models
        .into_iter()
        .map(model_to_sql)
        .collect::<Vec<String>>()
        .join("\n")
}

fn model_to_sql(model: Model) -> String {
    format!(
        "CREATE TABLE {}{};\n",
        model.name.to_lowercase(),
        vars_to_sql(model.vars)
    )
}

fn vars_to_sql(vars: Vec<Box<Variable>>) -> String {
    if vars.is_empty() {
        return "".to_string();
    }

    format!(
        "(\n\tid INT PRIMARY KEY,\n{}\n)", // this is a hack for primary key support - need to support x-keys
        vars.into_iter()
            .map(|m| *m)
            .map(var_to_sql)
            .collect::<Vec<String>>()
            .join(",\n"),
    )
}

fn var_to_sql(var: Variable) -> String {
    format!("\t{} {}", var.name, var.variable_type.to_mysql())
}
