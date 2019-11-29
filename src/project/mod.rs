use openapiv3::*;
use std::collections::HashMap;
use url::Url;

pub mod model;
pub use model::*;
pub mod variable;
pub use variable::*;
pub mod request;
pub use request::*;

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



use crate::error::*;

fn extract_variable_from_openapi(class_name: &str, var_name: &str, schema: openapiv3::Schema) -> CliResult<Variable> {
    if let openapiv3::SchemaKind::Type(schema_type) = schema.schema_kind {
        let variable_type = match schema_type {
            Type::String(_) => VariableType::StringType,
            Type::Number(_) => VariableType::FloatType,
            Type::Integer(_) => VariableType::IntType,
            // Type::Object(_) => {
            //     VariableType::ComplexType("Need to implement".to_string())
            // } //Need to implement
            Type::Array(val) => {
                let item_type = Project::parse_type(val.items.clone().unbox());
                VariableType::ArrayType(Box::new(item_type))
            }
            Type::Boolean {} => VariableType::BoolType,
            _ => {
            return Err(failure::format_err!(
                "Unsupported variable type on {} for {}: {:?}", class_name, var_name, schema_type));
            }
        };

        Ok(Variable {
            name: var_name.to_string(),
            optional: false,
            value: None,
            variable_type: variable_type
        })
    } else {
        Err(failure::format_err!(
            "Unsupported variable type on {} for {}", class_name, var_name))
    }
}

impl Project {
    pub fn parse_model(name: String, schema: openapiv3::Schema) -> CliResult<Model> {
        

        if let openapiv3::SchemaKind::Type(schema_type) = schema.schema_kind {
            // we only support single-type return types right now. (no multiple schema)

            if let Type::Object(o) = schema_type {
                // should be an object (will support raw return types later)

                let mut vars: Vec<Box<Variable>> = vec![];
                for (var_name, props) in o.properties {

                    if let ReferenceOr::Item(schema) = props {
                        vars.push(Box::new(extract_variable_from_openapi(&name, &var_name, *schema)?));
                    } else {
                        return Err(failure::format_err!(
                            "Reference types for variables are not supported in {} for {}", name, var_name))
                    }

                    // cycle through member variables
                    // println!("VAR FOUND: {:#?}", props);

                    // if let ReferenceOr::Item(schema) = props {
                    //     // okay, the variable wasn't a reference, lets find out what it was.

                    //     let var_type = if let openapiv3::SchemaKind::Type(schema_type) = schema.schema_kind {
                    //         match schema_type {
                    //             Type::String(_) => VariableType::StringType,
                    //             Type::Number(_) => VariableType::FloatType,
                    //             Type::Integer(_) => VariableType::IntType,
                    //             // Type::Object(_) => {
                    //             //     VariableType::ComplexType("Need to implement".to_string())
                    //             // } //Need to implement
                    //             Type::Array(val) => {
                    //                 let item_type = Project::parse_type(val.items.clone().unbox());
                    //                 VariableType::ArrayType(Box::new(item_type))
                    //             }
                    //             Type::Boolean {} => VariableType::BoolType,
                    //             _ => {
                    //             return Err(failure::format_err!(
                    //                 "Unsupported variable type on {} for {}: {:?}", name, var_name, schema_type));
                    //             }
                    //         };
                    //     } else {
                    //         return Err(failure::format_err!(
                    //             "Unsupported variable type on {} for {}", name, var_name));
                    //     };
                    //     // vars.push(Box::new(VariableType::StringType));
                    //     println!("VAR FOUND: {:?}", var_type);

                    // } else {
                    //     return Err(failure::format_err!(
                    //         "Reference types for variables are not supported in {} for {}", name, var_name));
                    // }

                }

                

                println!("model found: {:#?}", (name.clone(), vars.clone()));

                return Ok(Model { name, vars });
            } else {
                log::error!("Only concrete object types are supported as return types. model was: {}, schema_type was: {:?}", name, schema_type)
            }
        } else {
               log::error!("multiple inheritance schemas not supported! Please remove the use of OneOf, AllOf, etc. model was: {}, schema_kind was: {:?}", name, schema.schema_kind)
        }

        panic!("error supporting models in openapi.yml");




        // let schema_refs = match schema.schema_kind {
        //     openapiv3::SchemaKind::OneOf{ one_of } => one_of,
        //     openapiv3::SchemaKind::AllOf{ all_of } => all_of,
        //     openapiv3::SchemaKind::AnyOf{ any_of } => any_of,
        //     openapiv3::SchemaKind::Any(any) => any.properties,
        // };

        // if let openapiv3::SchemaKind::Any(any_schema) = schema.schema_kind {
        //     for (name, reference) in any_schema.properties {
        //         let optional = any_schema.required.contains(&name);
        //         let variable_type = Project::parse_type(reference.unbox());
        //         let variable = Variable {
        //             name,
        //             variable_type,
        //             optional: !optional,
        //             value: None,
        //         };

        //         vars.push(Box::new(variable));
        //     }
        // } else {
        //     if let openapiv3::SchemaKind::Type(schema_type) = schema.schema_kind {
        //         // let variable = ;

        //         // println!("TYPE: {:#?}", schema_type.properties);

        //         type_to_variable(schema_type);

                

        //     } else {
        //        log::error!("multiple inheritance schemas not supported! Please remove the use of OneOf, AllOf, etc. model was: {}, schema_kind was: {:?}", name, schema.schema_kind)
        //     }
        // }
        // Model { name, vars }
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
            ParameterSchemaOrContent::Content(_content) => {
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
                            ReferenceOr::Reference { reference } => {
                                reference.split('/').last().unwrap_or("").to_string()
                            }
                            _ => "".to_string(),
                        })
                        .unwrap_or_else(|| "".to_string())
                })
                .unwrap_or_else(|| "".to_string()),
            ReferenceOr::Reference { reference } => {
                reference.split('/').last().unwrap_or("").to_string()
            }
        }
    }

    fn parse_type(reference: ReferenceOr<openapiv3::Schema>) -> VariableType {
        match reference {
            ReferenceOr::Reference { reference } => {
                VariableType::ComplexType(reference.split('/').last().unwrap_or("").to_string())
            }
            ReferenceOr::Item(schema) => {
                match &schema.schema_kind {
                    openapiv3::SchemaKind::Type(t) => {
                        match t {
                            Type::String(_) => VariableType::StringType,
                            Type::Number(_) => VariableType::FloatType,
                            Type::Integer(_) => VariableType::IntType,
                            Type::Object(_) => {
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

    pub fn parse_yml(open_api: OpenAPI) -> CliResult<Self> {
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
            .unwrap_or_else(|| "".to_string());
        let res = Url::parse(url.as_str());

        if let Ok(url) = res {
            let scheme = url.scheme().to_string();
            let host = url.host_str().unwrap_or("");
            project.info = Info {
                host: (scheme + "://" + host),
                endpoint: url.path().to_string(),
            }
        };

        let mut arr_types = HashMap::new();
        //Parse models

        let components = open_api.components.unwrap();
        for (name, schema) in components.schemas {
            if let ReferenceOr::Item(schema) = schema {
                let mut is_array_type = false;
                if let openapiv3::SchemaKind::Type(type_) = schema.schema_kind.clone() {
                    if let Type::Array(array_type) = type_ {
                        let item_type = Project::parse_type(array_type.items.unbox());
                        if let VariableType::ComplexType(reference) = item_type {
                            arr_types.insert(name.clone(), reference);
                            is_array_type = true
                        }
                    };
                }
                if !is_array_type {
                    let model = Project::parse_model(name, schema)?;
                    project.models.push(model);
                }
            }
        }

        //Parse Requests
        for (url_path, path) in open_api.paths {
            if let ReferenceOr::Item(path_item) = path {
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
                        .map(Project::parse_response)
                        .unwrap_or_else(|| "ResponseEmpty".to_string());

                    let mut response_type = operation
                        .responses
                        .responses
                        .values()
                        .next()
                        .map(|response| Project::parse_response(response.clone()))
                        .map(|response| {
                            if response.chars().count() == 0 {
                                "ResponceEmpty".to_string()
                            } else {
                                response
                            }
                        })
                        .unwrap_or_else(|| "ResponceEmpty".to_string());
                    if arr_types.contains_key(&response_type) {
                        response_type = format!("[{}]", arr_types[&response_type].clone());
                        // println!("{}",arr_types[&response_type].clone());
                    }

                    let name = format!("{}{}request", &url_path, &method)
                        .replace("/", "")
                        .replace("{", "")
                        .replace("}", "");

                    let request = Request {
                        name,
                        vars,
                        path: split_url_path(&url_path),
                        method,
                        response_type,
                        error_type,
                    };
                    project.requests.push(request);
                }
            };
        }

        // println!("{:?}", project);

        Ok(project)
    }
}

trait Additional {
    fn path_to_request(&self) -> Vec<(Operation, Method)>;
}

impl Additional for PathItem {
    fn path_to_request(&self) -> Vec<(Operation, Method)> {
        let mm = self.clone();
        let arr: Vec<(Option<Operation>, Method)> = vec![
            (mm.get, Method::Get_),
            (mm.post, Method::Post_),
            (mm.put, Method::Put_),
            (mm.delete, Method::Delete_),
            (mm.options, Method::Options_),
            (mm.head, Method::Head_),
            (mm.patch, Method::Patch_),
            (mm.trace, Method::Trace_),
        ];

        arr.into_iter()
            .filter(|i| i.0.is_some())
            .map(|i| (i.0.unwrap(), i.1))
            .collect()
    }
}

pub trait CustomIterators {
    fn all_names(&self) -> Vec<String>;
}
impl CustomIterators for Vec<Model> {
    fn all_names(&self) -> Vec<String> {
        self.iter().map(|model| model.name.clone()).collect()
    }
}
impl CustomIterators for Vec<Request> {
    fn all_names(&self) -> Vec<String> {
        self.iter().map(|request| request.name.clone()).collect()
    }
}

fn split_url_path(url: &str) -> String {
    let mut url = url.split("/").collect::<Vec<_>>();
    url.remove(url.len() - 1);
    url.join("/")
}
