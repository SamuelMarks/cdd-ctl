#![cfg(not(coverage))]
#![cfg(not(tarpaulin_include))]
#![deny(missing_docs)]
#![warn(missing_docs)]

//! cdd-ctl: Daemon manage >13 processes and act as API gateway and authentication layer.
#![allow(unused_imports)]

use actix_web::{web, App, HttpServer};
use cdd_ctl::{api, db};
use clap::{Parser, Subcommand};
use log::{error, info};
use std::process::Command;
use std::sync::Arc;
use tokio::io::{stdin, stdout, AsyncBufReadExt, AsyncWriteExt, BufReader};

use cdd_ctl::AppConfig;
use cdd_ctl::{CddRepository, PgRepository};
use cdd_ctl::{GitHubClient, ReqwestGitHubClient};
use cdd_engine::daemon::{ProcessConfig, ProcessManager};
use cdd_engine::mcp::{McpOrchestrator, McpRequest, McpResponse};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long)]
    config: Option<String>,

    #[arg(short, long)]
    bind: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(name = "to_docs_json")]
    ToDocsJson {
        target_language: String,
        #[arg(short, long)]
        input: String,
        #[arg(long)]
        no_imports: bool,
        #[arg(long)]
        no_wrapping: bool,
    },
    #[command(name = "from_openapi")]
    FromOpenApi {
        target_language: String,
        target: String,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    #[command(name = "to_openapi")]
    ToOpenApi {
        target_language: String,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    #[command(name = "mcp")]
    Mcp { target_language: Option<String> },
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let args = Args::parse();

    match args.command {
        Some(Commands::ToDocsJson {
            target_language,
            input,
            no_imports,
            no_wrapping,
        }) => {
            let target = if target_language.starts_with("cdd-") {
                target_language.clone()
            } else {
                format!("cdd-{}", target_language)
            };
            let mut cmd = Command::new(&target);
            cmd.arg("to_docs_json");
            cmd.arg("-i").arg(&input);
            if no_imports {
                cmd.arg("--no-imports");
            }
            if no_wrapping {
                cmd.arg("--no-wrapping");
            }

            let output = cmd.output().unwrap_or_else(|e| {
                eprintln!("Failed to execute {}: {}", target, e);
                std::process::exit(1);
            });

            if !output.status.success() {
                std::io::Write::write_all(&mut std::io::stderr(), &output.stderr)?;
                std::process::exit(output.status.code().unwrap_or(1));
            }
            std::io::Write::write_all(&mut std::io::stdout(), &output.stdout)?;
            return Ok(());
        }
        Some(Commands::FromOpenApi {
            target_language,
            target,
            args: extra_args,
        }) => {
            let executable = if target_language.starts_with("cdd-") {
                target_language.clone()
            } else {
                format!("cdd-{}", target_language)
            };
            let mut cmd = Command::new(&executable);
            cmd.arg("from_openapi").arg(&target);
            for arg in extra_args {
                cmd.arg(arg);
            }

            let output = cmd.output().unwrap_or_else(|e| {
                eprintln!("Failed to execute {}: {}", executable, e);
                std::process::exit(1);
            });

            if !output.status.success() {
                std::io::Write::write_all(&mut std::io::stderr(), &output.stderr)?;
                std::process::exit(output.status.code().unwrap_or(1));
            }
            std::io::Write::write_all(&mut std::io::stdout(), &output.stdout)?;
            return Ok(());
        }
        Some(Commands::ToOpenApi {
            target_language,
            args: extra_args,
        }) => {
            let executable = if target_language.starts_with("cdd-") {
                target_language.clone()
            } else {
                format!("cdd-{}", target_language)
            };
            let mut cmd = Command::new(&executable);
            cmd.arg("to_openapi");
            for arg in extra_args {
                cmd.arg(arg);
            }

            let output = cmd.output().unwrap_or_else(|e| {
                eprintln!("Failed to execute {}: {}", executable, e);
                std::process::exit(1);
            });

            if !output.status.success() {
                std::io::Write::write_all(&mut std::io::stderr(), &output.stderr)?;
                std::process::exit(output.status.code().unwrap_or(1));
            }
            std::io::Write::write_all(&mut std::io::stdout(), &output.stdout)?;
            return Ok(());
        }
        Some(Commands::Mcp { target_language }) => {
            if let Some(lang) = target_language {
                let executable = if lang.starts_with("cdd-") {
                    lang
                } else {
                    format!("cdd-{}", lang)
                };
                let mut cmd = Command::new(&executable);
                cmd.arg("mcp");

                let mut child = cmd.spawn().unwrap_or_else(|e| {
                    eprintln!("Failed to spawn {}: {}", executable, e);
                    std::process::exit(1);
                });
                let status = child.wait()?;
                std::process::exit(status.code().unwrap_or(1));
            } else {
                let mut app_config = match AppConfig::load(args.config.as_deref()) {
                    Ok(c) => c,
                    Err(e) => {
                        error!("Failed to load configuration: {}", e);
                        std::process::exit(1);
                    }
                };

                if app_config.servers.is_empty() {
                    let native_tools = [
                        "cdd-c",
                        "cdd-cpp",
                        "cdd-csharp",
                        "cdd-go",
                        "cdd-java",
                        "cdd-kotlin",
                        "cdd-php",
                        "cdd-python",
                        "cdd-python-all",
                        "cdd-ruby",
                        "cdd-rust",
                        "cdd-sh",
                        "cdd-swift",
                        "cdd-ts",
                    ];
                    for tool in native_tools {
                        app_config.servers.insert(
                            tool.to_string(),
                            ProcessConfig {
                                command: Some(tool.to_string()),
                                args: Some(vec!["mcp".to_string()]),
                                external_address: None,
                                max_retries: 5,
                                restart_delay_ms: 2000,
                            },
                        );
                    }
                }

                let process_manager = Arc::new(ProcessManager::new(app_config.servers.clone()));
                if let Err(e) = process_manager.start_all().await {
                    error!("Error starting MCP processes: {}", e);
                    std::process::exit(1);
                }

                let mut stdin_reader = BufReader::new(stdin()).lines();
                let mut stdout_writer = stdout();

                while let Ok(Some(line)) = stdin_reader.next_line().await {
                    if let Ok(req) = serde_json::from_str::<McpRequest>(&line) {
                        let response = process_manager.handle_request(req).await;
                        let res_json = match response {
                            Ok(res) => serde_json::to_string(&res).unwrap_or_default(),
                            Err(e) => serde_json::to_string(&serde_json::json!({
                                "jsonrpc": "2.0",
                                "error": { "message": e.to_string() }
                            }))
                            .unwrap_or_default(),
                        };
                        let msg = format!("{}\n", res_json);
                        let _ = stdout_writer.write_all(msg.as_bytes()).await;
                    }
                }

                process_manager.stop_all().await;
                return Ok(());
            }
        }
        None => {}
    }

    let mut app_config = match AppConfig::load(args.config.as_deref()) {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    if let Some(bind) = args.bind {
        app_config.server_bind = bind;
    }

    if app_config.servers.is_empty() {
        info!("No servers configured, populating with default native dependencies.");
        let native_tools = [
            "cdd-c",
            "cdd-cpp",
            "cdd-csharp",
            "cdd-go",
            "cdd-java",
            "cdd-kotlin",
            "cdd-php",
            "cdd-python",
            "cdd-python-all",
            "cdd-ruby",
            "cdd-rust",
            "cdd-sh",
            "cdd-swift",
            "cdd-ts",
        ];
        for tool in native_tools {
            app_config.servers.insert(
                tool.to_string(),
                ProcessConfig {
                    command: Some(tool.to_string()),
                    args: Some(vec!["mcp".to_string()]),
                    external_address: None,
                    max_retries: 5,
                    restart_delay_ms: 2000,
                },
            );
        }
    }

    info!("Starting cdd-ctl server on {}", app_config.server_bind);

    let process_manager = Arc::new(ProcessManager::new(app_config.servers.clone()));

    let pm_clone = process_manager.clone();
    if let Err(e) = pm_clone.start_all().await {
        error!("Error starting processes: {}", e);
    }

    // Connect to PG Database
    let pool = db::establish_connection_pool(&app_config.database_url);
    let repo = Arc::new(PgRepository { pool });

    // Configure GitHub Client
    let github_client = Arc::new(ReqwestGitHubClient::new(
        std::env::var("GITHUB_CLIENT_ID").unwrap_or_default(),
        std::env::var("GITHUB_CLIENT_SECRET").unwrap_or_default(),
    ));

    let bind_addr = app_config.server_bind.clone();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(repo.clone() as Arc<dyn CddRepository>))
            .app_data(web::Data::new(
                github_client.clone() as Arc<dyn GitHubClient>
            ))
            .configure(api::configure)
    })
    .bind(&bind_addr)?
    .run();

    let result = server.await;

    // Shutdown processes
    process_manager.stop_all().await;

    result
}
