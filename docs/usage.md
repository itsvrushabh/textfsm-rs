# Usage Guide

## Installation

Add `textfsm-rs` to your `Cargo.toml`. Since this is a library, you usually depend on it:

```toml
[dependencies]
textfsm-rs = { path = "." } # Or git repository
serde_yml = "0.0.12" # Recommended for YAML serialization
```

## Basic Parsing

To parse a raw text file using a TextFSM template:

```rust
use textfsm_rs::{TextFSM, DataRecordConversion};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Load the template
    let mut fsm = TextFSM::from_file("path/to/template.textfsm")?;

    // 2. Parse the input content
    // Optional: Use DataRecordConversion::LowercaseKeys to normalize field names
    let results = fsm.parse_file("path/to/data.raw", Some(DataRecordConversion::LowercaseKeys))?;

    // 3. Process results
    for record in results {
        println!("{:?}", record.fields);
    }
    Ok(())
}
```

## Using CLI Table

The `CliTable` functionality allows automatic template selection based on the platform and command.

```rust
use textfsm_rs::CliTable;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the index file (usually 'index' in ntc-templates)
    let cli_table = CliTable::from_file("ntc_templates/templates/index")?;

    // Find a template
    if let Some((template_dir, row)) = cli_table.get_template_for_command("cisco_ios", "show version") {
        println!("Found template in: {}", template_dir);
        println!("Template files: {:?}", row.templates);
    }
    Ok(())
}
```

## Error Handling

The library uses a custom `TextFsmError` type (via `thiserror`). All major operations return a `Result`.

```rust
use textfsm_rs::TextFSM;

let fsm = match TextFSM::from_file("invalid.template") {
    Ok(f) => f,
    Err(e) => {
        eprintln!("Error loading template: {}", e);
        return;
    }
};
```
