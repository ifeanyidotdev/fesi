use std::{env, fs, path, process};

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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("{FESI_HELP_MESSAGE}");
        process::exit(1)
    }

    let sub_commands = &args[2..];
    //parse arguments
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-h" | "--help" => {
                println!("{FESI_HELP_MESSAGE}");
                process::exit(0)
            }
            "init" => {
                initialize_fesi_project();
            }
            "run" => {
                handle_run_command(sub_commands);
            }
            _ => {
                println!("{FESI_HELP_MESSAGE}");
                process::exit(0)
            }
        }
    }
}

fn handle_run_command(args: &[String]) {
    const RUN_HELP_MESSAGE: &str = r#"
Runs testing on the endpoint provided

Usage: fesi run [OPTIONS] <COMMAND>

Options:
    -m, --method        The HTTP method to run the test with (required)
    -e, --endpoint      The http url for the service to test (required) 
    -b, --body           JSON body for the request
    -hd, --header        Request header
    -h, --help        Prints the help message

Examples:
    fesi run -m GET -e http://localhost:8080/users
    fesi run -m POST -e http://localhost:8080/data -b '{"key": "value"}' -h "Content-Type: application/json" -h "Authorization: Bearer token"
"#;

    // let mut endpoint: Option<String> = None;
    // let mut method: Option<String> = None;
    // let mut body: Option<HashSet<String, String>> = None;
    // let mut header: Option<HashSet<String, String>> = None;
    //
    //
    // if args.len() <= 1 {
    //     println!("{RUN_HELP_MESSAGE}");
    //     process::exit(1);
    // }

    for arg in args.iter() {
        if arg == "-h" || arg == "--help" {
            println!("Testing");
            println!("{RUN_HELP_MESSAGE}");
            process::exit(1);
        }
    }
    //
    // for arg in args {
    //     let value = arg.as_str();
    //     match value {
    //         "-h" | "--help" => {
    //             println!("TESTING");
    //             println!("{RUN_HELP_MESSAGE}");
    //             process::exit(0)
    //         }
    //         _ => {
    //             println!("{RUN_HELP_MESSAGE}");
    //             process::exit(0)
    //         }
    //     }
    // }
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
