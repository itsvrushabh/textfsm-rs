# textfsm-rs

A robust, performant, and safe Rust implementation of [TextFSM](https://github.com/google/textfsm), designed for parsing semi-structured text (like network CLI output) into structured data.

## Features

-   **Full TextFSM Support**: Implements the core TextFSM grammar, including `Value` definitions, States, Rules, and Actions (`Next`, `Continue`, `Record`, etc.).
-   **CLI Table**: Built-in support for `ntc-templates` style index files for automatic template selection.
-   **Safety**: Strong error handling with `Result` and `thiserror` (no panics).
-   **Performance**: Optimized value insertion and state management.
-   **Modern**: Uses `serde` for serialization and `log` for observability.

## Documentation

-   [Usage Guide](docs/usage.md): How to use the library in your Rust projects.
-   [Architecture](docs/architecture.md): Internal design and implementation details.

## Quick Start

```rust
use textfsm_rs::TextFSM;

fn main() {
    let mut fsm = TextFSM::from_file("templates/cisco_ios_show_version.textfsm").unwrap();
    let records = fsm.parse_file("data/cisco_output.txt", None).unwrap();
    
    for record in records {
        println!("{:?}", record);
    }
}
```

## Testing

The project includes comprehensive tests against real-world data from `ntc-templates`.

```bash
cargo test
```

## License

Apache-2.0