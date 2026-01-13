# textfsm-rs

[![Rust](https://github.com/itsvrushabh/textfsm-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/itsvrushabh/textfsm-rs/actions/workflows/rust.yml)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)

A robust, performant, and safe Rust implementation of [TextFSM](https://github.com/google/textfsm). This library is designed to parse semi-structured text—specifically output from networking device CLI commands—into structured programmatic data (JSON, YAML, or Rust HashMaps).

## Why textfsm-rs?

*   **Blazing Fast**: Optimized for high-throughput parsing with minimal memory allocations.
*   **Safe by Design**: Written in 100% safe Rust, replacing Python's runtime errors with compile-time checks and graceful `Result` handling.
*   **NTC-Templates Compatible**: Designed to work out-of-the-box with the massive library of templates from [ntc-templates](https://github.com/networktocode/ntc-templates).
*   **Modern Tooling**: Seamless integration with `serde` for serialization and `pest` for reliable template parsing.

## Features

-   **Full TextFSM Specification**: Support for `Value` definitions (Filldown, Key, Required, List, Fillup), States, Rules, and Transitions.
-   **Advanced Regex**: Uses `fancy-regex` to support Python-style features like lookahead and lookbehind.
-   **Automatic Template Selection**: Built-in `CliTable` logic to automatically select the right template based on device platform and command.
-   **Zero-Panic**: Library code avoids `unwrap()` and `panic!`, ensuring your automation tools stay up.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
textfsm-rs = { git = "https://github.com/itsvrushabh/textfsm-rs.git" }
serde = { version = "1.0", features = ["derive"] }
```

## Quick Start

```rust
use textfsm_rs::{TextFSM, DataRecordConversion};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize the FSM with a template
    let mut fsm = TextFSM::from_file("templates/cisco_ios_show_version.textfsm")?;

    // 2. Parse your CLI output
    let records = fsm.parse_file("data/show_version.txt", Some(DataRecordConversion::LowercaseKeys))?;
    
    // 3. Use the structured data
    for record in records {
        if let Some(version) = record.fields.get("Version") {
            println!("Device Version: {}", version);
        }
    }

    Ok(())
}
```

## Advanced: Automated Template Mapping

Using the `ntc-templates` index style:

```rust
use textfsm_rs::CliTable;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let index = CliTable::from_file("ntc_templates/templates/index")?;
    
    // Automatically find the template for a Cisco command
    if let Some((dir, row)) = index.get_template_for_command("cisco_ios", "sh version") {
        println!("Match found! Template: {}/{}", dir, row.templates[0]);
    }
    
    Ok(())
}
```

## Documentation

*   [Usage Guide](docs/usage.md) - Detailed examples and API usage.
*   [Architecture](docs/architecture.md) - Deep dive into internal implementation and optimizations.

## Contributing

Contributions are welcome! Please ensure all tests pass:

```bash
cargo test
```

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.
