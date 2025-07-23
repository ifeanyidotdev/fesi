use std::{env, fs, path, process};

const FESI_HELP_MESSAGE: &str = r#"
Fesi is a drop in replacement for curl

Usage: fesi [OPTIONS] <COMMAND>

Commands:
    init   Initialize a new fesi project in the current direcotry

Optoons:
    -h, --help    Prints the help message
    -v, --version    Prints version informations
"#;

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-h" | "--help" => {
                println!("{FESI_HELP_MESSAGE}");
                process::exit(0)
            }
            "init" => {
                initialize_fesi();
            }
            _ => {
                println!("{FESI_HELP_MESSAGE}");
                process::exit(0)
            }
        }
    }
}

fn initialize_fesi() {
    if path::Path::new("FESI").is_dir() {
        eprintln!("Fesi project already initialized");
        process::exit(1)
    }
    fs::create_dir("FESI").unwrap_or_else(|err| {
        eprintln!("Could not create Fesi project, {err}");
        process::exit(1)
    });
    println!("Fesi project created");
    process::exit(0)
}
