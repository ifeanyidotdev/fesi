use std::{collections::HashMap, env, fs, path, process};

use crate::request::Request;

pub mod request;

const FESI_DIR_NAME: &str = "FESI";
const FESI_HELP_MESSAGE: &str = r#"
Fesi is a drop in replacement for curl

Usage: fesi [OPTIONS] <COMMAND>

Commands:
    init              Initialize a new fesi project in the current direcotry
    run               Run endpoint request

Optoons:
    -h, --help        Prints the help message
    -v, --version     Prints version informations
"#;
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
        _ => {
            println!("Error: Unknown command '{command}'");
            println!("{FESI_HELP_MESSAGE}");
            process::exit(1);
        }
    }
}

async fn handle_run_command(args: &[String]) {
    const RUN_HELP_MESSAGE: &str = r#"
Runs testing on the endpoint provided

Usage: fesi run [OPTIONS]

Options:
    -m, --method <METHOD>      The HTTP method to use (required)
    -e, --endpoint <URL>       The HTTP URL for the service to test (required)
    -b, --body <KEY=VALUE>     A key-value pair for the request body. Use multiple times for multiple values.
    -hd, --header <KEY=VALUE>  A key-value pair for the request header. Use multiple times for multiple headers.
    -h, --help                 Prints this help message

Examples:
    fesi run -m GET -e http://localhost:8080/users
    fesi run -m POST -e http://localhost:8080/data -b 'user=fesi' -b 'pass=supersecret' -hd 'Content-Type=application/json'
"#;

    if args.is_empty()
        || args.contains(&String::from("-h"))
        || args.contains(&String::from("--help"))
    {
        println!("{RUN_HELP_MESSAGE}");
        process::exit(0);
    }

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
                println!("{RUN_HELP_MESSAGE}");
                process::exit(1);
            }
        }
    }

    if method.is_none() {
        eprintln!("Error: --method is required");
        println!("{RUN_HELP_MESSAGE}");
        process::exit(1);
    }

    if endpoint.is_none() {
        eprintln!("Error: --endpoint is required");
        println!("{RUN_HELP_MESSAGE}");
        process::exit(1);
    }
    let request_value = Request {
        method: method.clone().unwrap(),
        endpoint: endpoint.unwrap(),
        header: headers,
        body,
    };
    if let Some(mth) = method {
        match mth.as_str() {
            "GET" => {
                let res = request_value.get().await.unwrap_or_else(|err| {
                    eprintln!("{err}");
                    process::exit(1);
                });
                println!("{}", res.as_str());
                process::exit(0);
            }
            _ => println!("Method not allowed yet"),
        }
    } else {
        eprintln!("Error: --method is required");
        println!("{RUN_HELP_MESSAGE}");
        process::exit(1);
    }
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
}
