# [rk_pin](https://crates.io/crates/rk_pin)

GPIO Converter tools for Rockchip

## Description

This Rust program calculates and converts between Rockchip GPIO pin names and pin numbers. It is useful for developers and engineers working with Rockchip hardware, who need to map GPIO pin names to their corresponding numbers and vice versa.

## Features

- Convert GPIO pin names (e.g., `GPIO4_C5`) to pin numbers.
- Convert pin numbers back to GPIO pin names.

## Usage

### Command Line Arguments

The program accepts two types of command line arguments:

- `-p, --pin <PIN_NAME>`: Convert a GPIO pin name to a pin number.
- `-n, --number <PIN_NUMBER>`: Convert a pin number to a GPIO pin name.

### Examples

To convert a GPIO pin name to a pin number:

```bash
cargo run -- -p GPIO4_C5
```

Output:

```bash
GPIO_NUM: 21
Pin name GPIO4_C5 corresponds to pin number: 149
```

To convert a pin number to a GPIO pin name:

```bash
cargo run -- -n 149
```

Output:

```bash
Pin number 149 corresponds to: [GPIO4_C5]
```

### Building

To build the project, use the following command:

```bash
cargo build --release
```

The optimized binary will be located in the target/release/ directory.

After building the project, you can copy the binary to a directory in your PATH or distribute it as needed.

### Installation

You can also use the `cargo install` command to install the program globally.

```bash
cargo install rk_pin
```
