# dechex

A simple command-line tool to convert between decimal and hexadecimal numbers.

## Features

- **Automatic conversion**: Automatically detects the input format and converts accordingly
- **Hex mode** (`-x`/`--hex`): Convert decimal to hex, or return hex as-is
- **Dec mode** (`-d`/`--dec`): Convert hex to decimal, or return decimal as-is
- Fast and lightweight

## Installation

### From Source

Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed, then:

```bash
git clone <repository-url>
cd dechex
cargo build --release
```

The binary will be available at `target/release/dechex`.

## Usage

```bash
dechex <VALUE> [OPTIONS]
```

### Options

- `-x, --hex`: Convert decimal to hex, return hex as-is (error on other bases)
- `-d, --dec`: Convert hex to decimal, return decimal as-is (error on other bases)
- `-h, --help`: Print help information
- `-V, --version`: Print version information

### Examples

#### Automatic mode (no flags)

Convert hexadecimal to decimal:
```bash
$ dechex 0x1F
31
```

Convert decimal to hexadecimal:
```bash
$ dechex 255
0xFF
```

#### Hex mode (`-x`)

Convert decimal to hex:
```bash
$ dechex -x 255
0xFF
```

Return hex as-is:
```bash
$ dechex -x 0xFF
0xFF
```

#### Dec mode (`-d`)

Convert hex to decimal:
```bash
$ dechex -d 0x1F
31
```

Return decimal as-is:
```bash
$ dechex -d 255
255
```

## Error Handling

The tool will return an error and exit with status code 1 in the following cases:

- Invalid number format (e.g., `abc` without `0x` prefix)
- Invalid hexadecimal number (e.g., `0x01WE`)
- Using non-decimal/non-hex formats in mode-specific operations (e.g., `dechex -x 0b1010`)

## Running Tests

```bash
cargo test
```
