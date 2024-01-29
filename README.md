# Taskmaster

Taskmaster is a lightweight process manager developed in Rust, offering a simplified version of supervisord. It aims to provide basic process management functionalities with a minimal footprint. The project leverages only two dependencies: `serde` and `serde_yaml` for parsing the configuration file.

## Features

- **Process Management**: Taskmaster allows users to manage processes efficiently, including starting, stopping, and restarting them as needed.
  
- **Configuration via YAML**: Configuration for Taskmaster can be done using YAML files, providing a simple and flexible way to define process parameters and settings.

## Installation

To install Taskmaster, you need to have Rust installed on your system. Once Rust is installed, you can build the project from source:

```bash
git clone https://github.com/Laurent45/taskmaster.git
cd taskmaster
cargo build --release
```
This will generate the executable binary in the `target/release` directory.

## Usage
To use Taskmaster, you need to create a configuration file in YAML format. Follow this template to create one:
```yaml
processes:
  - name: example_process
    command: /path/to/executable
    args:
      - arg1
      - arg2
    autostart: true
```
Once you have your configuration file ready, you can start Taskmaster with:

```bash
	$>taskmaster /path/to/config.yaml
```

Taskmaster will then read the configuration file and manage the processes accordingly.

### Command Line Options

-   `-h, --help`: Display help information about command line options.
-   `-V, --version`: Display version information.

## Contributing

Contributions to Taskmaster are welcome! If you find bugs or have suggestions for improvements, please open an issue on the GitHub repository or submit a pull request.

## License
