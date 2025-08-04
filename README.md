# Fesi

Fesi is a CLI tool for interacting with your application's REST endpoints. It allows you to send HTTP requests, organize your requests in files, and save the responses.

## Installation

To install Fesi, you need to have Rust and Cargo installed on your system. You can then install Fesi using the following command:

```bash
cargo install fesi
```

## Basic Usage

Fesi provides several commands to interact with your REST endpoints.

### `init`

The `init` command initializes a new Fesi project. This creates a `.fesi` directory in the current directory, where you can store your request files.

```bash
fesi init
```

### `run`

The `run` command allows you to execute a single HTTP request.

```bash
fesi run -m <METHOD> -e <ENDPOINT> [OPTIONS]
```

**Arguments:**

*   `-m`, `--method`: The HTTP method (e.g., `GET`, `POST`, `PUT`, `DELETE`).
*   `-e`, `--endpoint`: The URL endpoint for the request.

**Options:**

*   `-b`, `--body`: The request body in `key=value` format. You can specify this option multiple times for multiple key-value pairs.
*   `-hd`, `--header`: The request headers in `key=value` format. You can specify this option multiple times for multiple key-value pairs.

**Example:**

```bash
fesi run -m GET -e https://api.example.com/users/1
```

```bash
fesi run -m POST -e https://api.example.com/users -b "name=John Doe" -b "email=john.doe@example.com"
```

### `file`

The `file` command allows you to execute a series of requests defined in a YAML file.

```bash
fesi file <FILE_PATH>
```

**Argument:**

*   `<FILE_PATH>`: The path to the YAML file containing the request definitions.

**Example `requests.yaml`:**

```yaml
actions:
  - name: "Get User"
    method: "GET"
    endpoint: "https://api.example.com/users/1"
  - name: "Create User"
    method: "POST"
    endpoint: "https://api.example.com/users"
    body:
      name: "Jane Doe"
      email: "jane.doe@example.com"
```

**Command:**

```bash
fesi file requests.yaml
```

This will execute the "Get User" request first, followed by the "Create User" request. The responses will be saved to files in the current directory.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue on the project's repository.