use openapiv3::{OpenAPI, ReferenceOr};

mod model;
pub use model::*;
mod variable;
pub use variable::*;
mod request;
use crate::project::request::Method::*;
use openapiv3::*;
pub use request::*;
use std::collections::HashMap;
use url::Url;

#[derive(Debug)]
pub struct Project {
    pub info: Info,
    pub models: Vec<Model>,
    pub requests: Vec<Request>,
}

#[derive(Debug)]
pub struct Info {
    pub host: String,
    pub endpoint: String,
}

impl Project {
    pub fn parse_model(name: String, schema: openapiv3::Schema) -> Model {
        let mut vars: Vec<Box<Variable>> = vec![];
        if let openapiv3::SchemaKind::Any(any_schema) = schema.schema_kind {
            for (name, reference) in any_schema.properties {
                let optional = any_schema.required.contains(&name);
                let variable_type = Project::parse_type(reference.unbox());
                let variable = Variable {
                    name,
                    variable_type,
                    optional: !optional,
                    value: None,
                };

                vars.push(Box::new(variable));
            }
        }
        Model { name, vars }
    }

    fn parse_parameter_data(data: ParameterData) -> Variable {
        match data.format {
            ParameterSchemaOrContent::Schema(reference) => {
                let variable_type = Project::parse_type(reference);
                Variable {
                    name: data.name,
                    variable_type,
                    optional: !data.required,
                    value: None,
                }
            }
            ParameterSchemaOrContent::Content(content) => {
                //Need to implement
                Variable {
                    name: data.name,
                    variable_type: VariableType::StringType,
                    optional: false,
                    value: None,
                }
            }
        }
    }

    /// parse response string from openapi
    fn parse_response(response: ReferenceOr<Response>) -> String {
        match response {
            ReferenceOr::Item(response) => response
                .content
                .values()
                .next()
                .map(|media_type| {
                    media_type
                        .schema
                        .clone()
                        .map(|schema| match schema {
                            ReferenceOr::Reference { reference } => reference,
                            _ => "".to_string(),
                        })
                        .unwrap_or("".to_string())
                })
                .unwrap_or("".to_string()),
            ReferenceOr::Reference { reference } => reference,
        }
    }

    fn parse_type(reference: ReferenceOr<openapiv3::Schema>) -> VariableType {
        match reference {
            ReferenceOr::Reference { reference } => {
                VariableType::ComplexType(reference.split("/").last().unwrap_or("").to_string())
            }
            ReferenceOr::Item(schema) => {
                match &schema.schema_kind {
                    openapiv3::SchemaKind::Type(t) => {
                        match t {
                            Type::String(val) => VariableType::StringType,
                            Type::Number(val) => VariableType::FloatType,
                            Type::Integer(val) => VariableType::IntType,
                            Type::Object(val) => {
                                VariableType::ComplexType("Need to implement".to_string())
                            } //Need to implement
                            Type::Array(val) => {
                                let item_type = Project::parse_type(val.items.clone().unbox());
                                VariableType::ArrayType(Box::new(item_type))
                            }
                            Type::Boolean {} => VariableType::BoolType,
                        }
                    }
                    _ => VariableType::StringType,
                }
            }
        }
    }

    pub fn parse_yml(open_api: OpenAPI) -> Self {
        // println!("{}", open_api.info.title);
        //Parse INFO
        let mut project = Project {
            info: Info {
                host: "".to_string(),
                endpoint: "".to_string(),
            },
            models: vec![],
            requests: vec![],
        };
        let url = open_api
            .servers
            .first()
            .map(|s| s.url.clone())
            .unwrap_or("".to_string());
        let res = Url::parse(url.as_str());
        match res {
            Ok(url) => {
                let scheme = url.scheme().to_string();
                let host = url.host_str().unwrap_or("");
                project.info = Info {
                    host: (scheme + "://" + host),
                    endpoint: url.path().to_string(),
                }
            }
            Err(err) => {}
        };

        let mut arrTypes = HashMap::new();
        //Parse models

        let components = open_api.components.unwrap();
        for (name, schema) in components.schemas {
            match schema {
                ReferenceOr::Item(schema) => {
                    if let openapiv3::SchemaKind::Type(type_) = schema.schema_kind.clone() {
                        if let Type::Array(array_type) = type_ {
                            let item_type = Project::parse_type(array_type.items.unbox());
                            if let VariableType::ComplexType(reference) = item_type {
                                arrTypes.insert(name.clone(), reference);
                                // println!("---------{}", arrTypes);
                            }
                        };
                    }

                    let model = Project::parse_model(name, schema);
                    project.models.push(model);
                }
                ReferenceOr::Reference { reference } => {} //Need to implement
            }
        }

        //Parse Requests

        for (url_path, path) in open_api.paths {
            match path {
                ReferenceOr::Item(path_item) => {
                    for (operation, method) in path_item.path_to_request() {
                        let mut vars: Vec<Box<Variable>> = vec![];

                        for ref_or_parameter in operation.parameters {
                            if let ReferenceOr::Item(parameter) = ref_or_parameter {
                                match parameter {
                                    Parameter::Query {
                                        parameter_data,
                                        allow_reserved,
                                        style,
                                        allow_empty_value,
                                    } => {
                                        vars.push(Box::new(Project::parse_parameter_data(
                                            parameter_data,
                                        )));
                                    }
                                    Parameter::Path {
                                        parameter_data,
                                        style,
                                    } => {
                                        vars.push(Box::new(Project::parse_parameter_data(
                                            parameter_data,
                                        )));
                                    }

                                    _ => {} //Need to finish
                                }
                            }
                        }

                        let error_type = operation
                            .responses
                            .default
                            .map(|default_response| Project::parse_response(default_response))
                            .unwrap_or("".to_string());

                        let response_type = operation
                            .responses
                            .responses
                            .values()
                            .next()
                            .map(|response| Project::parse_response(response.clone()))
                            .unwrap_or("".to_string());

                        let name = "randomName".to_string();
                        let request = Request {
                            name,
                            vars,
                            path: url_path.to_string(),
                            method,
                            response_type,
                            error_type,
                        };
                        project.requests.push(request);
                    }
                }
                _ => {} //Need to implement
            };
        }

        // println!("{:?}", project);

        return project;
    }
}

trait Additional {
    fn path_to_request(&self) -> Vec<(Operation, Method)>;
}
impl Additional for PathItem {
    fn path_to_request(&self) -> Vec<(Operation, Method)> {
        let mm = self.clone();
        let arr: Vec<(Option<Operation>, Method)> = vec![
            (mm.get, Get_),
            (mm.post, Post_),
            (mm.put, Put_),
            (mm.delete, Delete_),
            (mm.options, Options_),
            (mm.head, Head_),
            (mm.patch, Patch_),
            (mm.trace, Trace_),
        ];

        let mut res: Vec<(Operation, Method)> = arr
            .into_iter()
            .filter(|i| i.0.is_some())
            .map(|i| (i.0.unwrap(), i.1))
            .collect();

        res
    }
}

pub trait CustomIterators {
    fn all_names(&self) -> Vec<String>;
}
impl CustomIterators for Vec<Model> {
    fn all_names(&self) -> Vec<String> {
        self.into_iter().map(|model| model.name.clone()).collect()
    }
}
impl CustomIterators for Vec<Request> {
    fn all_names(&self) -> Vec<String> {
        self.into_iter()
            .map(|request| request.name.clone())
            .collect()
    }
}

// impl From<openapiv3::OpenAPI> for Project {
//     fn from(openapi: openapiv3::OpenAPI) -> Self {
//         info!("Extracting objects from OpenAPI spec");
//         let mut models = Vec::new();
//         let mut requests = Vec::new();

//         for component in openapi.components {
//             for (name, reference) in component.schemas {
//                 println!("class found: {}", name);

//                 if let openapiv3::ReferenceOr::Item(schema) = reference {
//                     if let openapiv3::SchemaKind::Any(schema_kind) = schema.schema_kind {
//                         // variables are here:
//                         for (name, property) in schema_kind.properties {
//                             println!("found variable: {}", name);
//                             // println!("{:#}", property);
//                             read_variable(name, property);
//                         }
//                     }
//                 }
//             }
//         }

//         for (path, item) in openapi.paths {
//             if let openapiv3::ReferenceOr::Item(item) = item {
//                 fn make_route(
//                     method: &str,
//                     url_path: &str,
//                     operation: &openapiv3::Operation,
//                 ) -> Request {
//                     Request {
//                         method: Method::Get_,
//                         error_type: "".to_string(),
//                         response_type: "".to_string(),
//                         url: Url::new(),
//                         vars: Vec::new(),
//                     }
//                 }

//                 if let Some(operation) = item.get {
//                     // println!("GET: {:#?}", operation);
//                     requests.push(make_route("GET", &path, &operation));
//                 }
//                 if let Some(operation) = item.put {
//                     requests.push(make_route("PUT", &path, &operation));
//                 }
//                 if let Some(operation) = item.post {
//                     requests.push(make_route("POST", &path, &operation));
//                 }
//                 if let Some(operation) = item.delete {
//                     requests.push(make_route("DELETE", &path, &operation));
//                 }
//             }
//         }

//         Project {
//             models,
//             requests,
//             info: Info {
//                 endpoint: "".to_string(),
//                 host: "".to_string(),
//             },
//         }
//     }
// }

// use crate::*;
// use openapiv3::*;

// fn read_variable(
//     name: std::string::String,
//     schema: ReferenceOr<std::boxed::Box<openapiv3::Schema>>,
// ) -> CliResult<Variable> {
//     // println!("schema: {:#?}", schema);
//     if let ReferenceOr::Item(schema) = schema {
//         let required = schema.schema_data.nullable;
//         let constant = schema.schema_data.read_only;

//         println!("schema: {:#?}", schema);

//         if let openapiv3::SchemaKind::Type(var_type) = schema.schema_kind {
//             println!("schema_kind: {:#?}", var_type);

//             let vartype = match var_type {
//                 openapiv3::Type::Integer { .. } => "Int64",
//                 _ => "Unknown",
//             };

//             return Ok(Variable {});
//         }
//         // let vartype = match schema. {
//         //     openapiv3::Type::Integer { .. } => "Int64",
//         //     _ => "Unknown",
//         // };
//     }
// }
