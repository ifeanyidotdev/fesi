pub const FESI_DIR_NAME: &str = "Fesi";
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

pub const RUN_HELP_MESSAGE: &str = r#"
Runs testing on the endpoint provided

Usage: fesi run [OPTIONS]

Options:
    -m, --method <METHOD>      The HTTP method to use (required)
    -e, --endpoint <URL>       The HTTP URL for the service to test (required)
    -b, --body <KEY=VALUE>     A key-value pair for the request body. Use multiple times for multiple values.
    -hd, --header <KEY=VALUE>  A key-value pair for the request header. Use multiple times for multiple headers.
    -h, --help                 Prints the help message

Examples:
    fesi run -m GET -e http://localhost:8080/users
    fesi run -m POST -e http://localhost:8080/data -b 'user=fesi' -b 'pass=supersecret' -hd 'Content-Type=application/json'
"#;
