pub const FESI_DIR_NAME: &str = "FESI";
pub const FESI_HELP_MESSAGE: &str = r#"
Fesi is a drop in replacement for curl

Usage: fesi [OPTIONS] <COMMAND>

Commands:
    init              Initialize a new fesi project in the current direcotry
    run               Run endpoint request
    file              Run from a yaml defined file

Options:
    -h, --help        Prints the help message
    -v, --version     Prints version informations
"#;
