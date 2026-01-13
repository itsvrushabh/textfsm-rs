use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_gen_yaml_file_integration() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let raw_file_path = temp_dir.path().join("cisco_ios_show_clock.raw");
    let expected_yml_path = "tests/cisco_ios/show_clock/cisco_ios_show_clock.yml";
    let original_raw_path = "tests/cisco_ios/show_clock/cisco_ios_show_clock.raw";

    fs::copy(original_raw_path, &raw_file_path)?;

    let mut cmd = Command::cargo_bin("textfsm-templates")?;
    cmd.arg("gen-yaml-file")
        .arg("--file")
        .arg(&raw_file_path);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Generating YAML file from"));

    let generated_yml_path = raw_file_path.with_extension("yml");
    let generated_yml_content = fs::read_to_string(generated_yml_path)?;
    let expected_yml_content = fs::read_to_string(expected_yml_path)?;

    assert_eq!(generated_yml_content, expected_yml_content);

    Ok(())
}