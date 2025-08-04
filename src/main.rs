mod constants;
mod parser;
mod request;
mod save;

use crate::{request::Request, save::save_response_to_file};
use std::{collections::HashMap, env, fs, future::Future, path, process};

use crate::constants::*;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("{FESI_HELP_MESSAGE}");
        process::exit(1);
    }

    let command = &args[1];
    let sub_commands = &args[2..].to_vec();

    match command.as_str() {
        "-h" | "--help" => {
            println!("{FESI_HELP_MESSAGE}");
            process::exit(0);
        }
        "init" => {
            initialize_fesi_project();
        }
        "run" => {
            handle_run_command(sub_commands).await;
        }
        "file" => {
            let file = sub_commands.iter().next().unwrap_or_else(|| {
                eprintln!("No file passed");
                process::exit(1);
            });
            let rest: parser::Rest = parser::load_rest_file(file).unwrap_or_else(|error| {
                eprintln!("{:?}", error.to_string());
                process::exit(1);
            });
            let requests = parser::parse_to_request(rest.actions).await;
            for r in requests {
                let result = r.run().await.unwrap_or_else(|error| {
                    eprintln!("{:?}", error.to_string());
                    process::exit(1);
                });
                save_response_to_file(result, r.name.unwrap_or("response".to_string()))
                    .unwrap_or_else(|error| {
                        eprintln!("{:?}", error.to_string());
                        process::exit(1);
                    });
            }
        }
        _ => {
            println!("Error: Unknown command '{command}'");
            println!("{FESI_HELP_MESSAGE}");
            process::exit(1);
        }
    }
}

async fn handle_run_command(args: &[String]) {
    if args.is_empty()
        || args.contains(&String::from("-h"))
        || args.contains(&String::from("--help"))
    {
        println!("{RUN_HELP_MESSAGE}");
        process::exit(0);
    }

    let request: Request = parse_run_command(args, RUN_HELP_MESSAGE).await;
    let method: String = request.method.clone();

    match method.as_str() {
        "GET" | "get" => {
            let res = request.get().await.unwrap_or_else(|err| {
                eprintln!("{err}");
                process::exit(1);
            });
            println!("{}", res.as_str());
            process::exit(0);
        }
        "POST" | "post" => {
            let res = request.post().await.unwrap_or_else(|err| {
                eprintln!("{err}");
                process::exit(1);
            });
            println!("{}", res.as_str());
            process::exit(0);
        }
        "PUT" | "put" => {
            let res = request.put().await.unwrap_or_else(|err| {
                eprintln!("{err}");
                process::exit(1);
            });
            println!("{}", res.as_str());
            process::exit(0);
        }
        "PATCH" | "patch" => {
            let res = request.patch().await.unwrap_or_else(|err| {
                eprintln!("{err}");
                process::exit(1);
            });
            println!("{}", res.as_str());
            process::exit(0);
        }
        "DELETE" | "delete" => {
            let res = request.delete().await.unwrap_or_else(|err| {
                eprintln!("{err}");
                process::exit(1);
            });
            println!("{}", res.as_str());
            process::exit(0);
        }
        _ => {
            eprintln!("method is required");
            println!("{RUN_HELP_MESSAGE}");
            process::exit(1);
        }
    }
}

fn parse_run_command(args: &[String], help_message: &str) -> impl Future<Output = Request> {
    let mut method: Option<String> = None;
    let mut endpoint: Option<String> = None;
    let mut body: HashMap<String, String> = HashMap::new();
    let mut headers: HashMap<String, String> = HashMap::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-m" | "--method" => {
                if i + 1 < args.len() {
                    method = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: Missing value for --method");
                    process::exit(1);
                }
            }
            "-e" | "--endpoint" => {
                if i + 1 < args.len() {
                    endpoint = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: Missing value for --endpoint");
                    process::exit(1);
                }
            }
            "-b" | "--body" => {
                if i + 1 < args.len() {
                    let part = &args[i + 1];
                    let mut splitter = part.splitn(2, '=');
                    if let (Some(key), Some(value)) = (splitter.next(), splitter.next()) {
                        body.insert(key.to_string(), value.to_string());
                    } else {
                        eprintln!(
                            "Error: Invalid format for --body. Expected key=value, got '{part}'"
                        );
                        process::exit(1);
                    }
                    i += 2;
                } else {
                    eprintln!("Error: Missing value for --body");
                    process::exit(1);
                }
            }
            "-hd" | "--header" => {
                if i + 1 < args.len() {
                    let part = &args[i + 1];
                    let mut splitter = part.splitn(2, '=');
                    if let (Some(key), Some(value)) = (splitter.next(), splitter.next()) {
                        headers.insert(key.to_string(), value.to_string());
                    } else {
                        eprintln!(
                            "Error: Invalid format for --header. Expected key=value, got '{part}'"
                        );
                        process::exit(1);
                    }
                    i += 2;
                } else {
                    eprintln!("Error: Missing value for --header");
                    process::exit(1);
                }
            }
            _ => {
                eprintln!("Error: Unknown argument '{}'", args[i]);
                println!("{help_message}");
                process::exit(1);
            }
        }
    }

    if method.is_none() {
        eprintln!("Error: --method is required");
        println!("{help_message}");
        process::exit(1);
    }

    if endpoint.is_none() {
        eprintln!("Error: --endpoint is required");
        println!("{help_message}");
        process::exit(1);
    }
    Request::new(
        method.clone().unwrap(),
        endpoint.unwrap(),
        body,
        headers,
        None,
    )
}

fn initialize_fesi_project() {
    if path::Path::new(FESI_DIR_NAME).is_dir() {
        eprintln!("Fesi project already initialized");
        process::exit(0)
    }
    fs::create_dir(FESI_DIR_NAME).unwrap_or_else(|err| {
        eprintln!("Could not create Fesi project, {err}");
        process::exit(1)
    });
    println!("Fesi project created");
    process::exit(0)
}

#[cfg(test)]
mod test {
    use super::*;

    fn clean_up_dir() {
        if path::Path::new(FESI_DIR_NAME).is_dir() {
            fs::remove_dir_all(FESI_DIR_NAME).unwrap();
        }
    }
    #[test]
    fn test_initlize_fesi_success() {
        clean_up_dir();
        let result = std::panic::catch_unwind(|| {
            initialize_fesi_project();
        });

        assert!(result.is_ok());
        assert!(path::Path::new(FESI_DIR_NAME).is_dir());

        clean_up_dir();
    }

    #[test]
    fn test_run_hanlde_success() {}
}
