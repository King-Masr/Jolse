# Poius

Poius is a command-line interface (CLI) tool written in Rust for encoding and decoding files and directories using base64 encoding.

## Features

- Encode files or directories to base64 format.
- Decode base64-encoded files or directories back to their original format.
- Simple and easy-to-use CLI interface.
- Supports both encoding and decoding operations.

## Installation

To use Poius, you'll need to have Rust installed on your system. You can install Rust by following the instructions on the [official Rust website](https://www.rust-lang.org/).

Once Rust is installed, you can clone this repository and build the project using Cargo, Rust's package manager and build system.

```bash
git clone https://github.com/King-Masr/Poius.git
cd Poius
cargo build --release
```

This will create the `poius` executable in the `target/release` directory.

## Usage

Poius provides a simple command-line interface with the following usage:

```
poius <command> <path>
```

Commands:

- `encode <file/dir>`: Encode a file or directory to base64 format.
- `decode <file/dir>`: Decode a base64-encoded file or directory.
- `help`: Display the help message.

Example usage:

```bash
# Encode a file
poius encode /path/to/file.txt

# Decode a file
poius decode /path/to/file.txt.encoded
```

## Documentation

For more detailed information about Poius and its usage, please refer to the [Documentation](https://docs.rs/poius).

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
