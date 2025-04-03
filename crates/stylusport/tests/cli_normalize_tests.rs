// In crates/stylusport/tests/cli_normalize_tests.rs
use assert_cmd::Command;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper to get the path to test fixtures
fn fixture_path(name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../../examples");
    path.push(name);
    path.push("lib.rs");
    path
}

/// Validates key normalized structure elements regardless of serialization format
fn validate_normalized_structure(content: &str) {
    // Skip validation if content is empty
    if content.trim().is_empty() {
        panic!("Empty output received, cannot validate structure");
    }

    // First try to parse as JSON
    let json_result = serde_json::from_str::<serde_json::Value>(content);

    // If JSON parsing fails, try YAML
    let value = match json_result {
        Ok(json) => json,
        Err(json_err) => {
            // Try YAML as fallback
            match serde_yaml::from_str::<serde_yaml::Value>(content) {
                Ok(yaml) => {
                    // Convert YAML to JSON string, then parse back to JSON Value
                    // This is a bit convoluted but allows us to use the same validation logic
                    let yaml_json = serde_json::to_string(&yaml)
                        .expect("Failed to convert YAML to JSON string");
                    serde_json::from_str(&yaml_json).expect("Failed to parse converted YAML-JSON")
                }
                Err(yaml_err) => {
                    panic!(
                        "Failed to parse content as either JSON ({}) or YAML ({})\nContent:\n{}",
                        json_err, yaml_err, content
                    );
                }
            }
        }
    };

    // Core structure validation
    assert!(
        value.get("name").is_some(),
        "Missing name in normalized output"
    );
    assert!(
        value.get("modules").is_some(),
        "Missing modules in normalized output"
    );
    assert!(
        value.get("account_structs").is_some(),
        "Missing account_structs in normalized output"
    );

    // Deeper validation of modules and their instructions
    if let Some(modules) = value.get("modules").and_then(|m| m.as_array()) {
        if !modules.is_empty() {
            let first_module = &modules[0];
            assert!(first_module.get("name").is_some(), "Module missing name");
            assert!(
                first_module.get("instructions").is_some(),
                "Module missing instructions"
            );

            // Check that there are instructions in at least one module
            let has_instructions = modules.iter().any(|module| {
                module
                    .get("instructions")
                    .and_then(|instrs| instrs.as_array())
                    .map(|instrs| !instrs.is_empty())
                    .unwrap_or(false)
            });

            assert!(has_instructions, "No modules found with instructions");
        }
    }

    // Validate account structs exist and have expected structure
    if let Some(account_structs) = value.get("account_structs").and_then(|a| a.as_array()) {
        if !account_structs.is_empty() {
            let first_account = &account_structs[0];
            assert!(
                first_account.get("name").is_some(),
                "Account struct missing name"
            );
            assert!(
                first_account.get("fields").is_some(),
                "Account struct missing fields"
            );
        }
    }
}

#[test]
fn test_normalize_file_yaml() {
    let fixture_path = fixture_path("hello_world");

    println!("Using fixture path: {:?}", fixture_path);

    // Build the command explicitly for debugging
    let mut cmd = Command::cargo_bin("stylusport").unwrap();
    cmd.arg("normalize")
        .arg(fixture_path.to_str().unwrap())
        .arg("--format=yaml");

    println!("Executing command: {:?}", cmd);

    // Execute and capture output
    let output = cmd.output().unwrap();

    assert!(output.status.success(), "Normalization should succeed");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Remove the log line
    let yaml_content = stdout
        .lines()
        .filter(|line| !line.contains("INFO"))
        .collect::<Vec<_>>()
        .join("\n");

    // For debugging
    println!("Raw YAML content:\n{}", yaml_content);

    // Validate structure
    validate_normalized_structure(&yaml_content);

    // Also verify we can parse as YAML
    let _parsed: serde_yaml::Value =
        serde_yaml::from_str(&yaml_content).expect("Failed to parse as YAML");

    // Snapshot testing
    insta::assert_snapshot!(yaml_content);
}

#[test]
fn test_normalize_file_json() {
    let fixture_path = fixture_path("hello_world");

    let output = Command::cargo_bin("stylusport")
        .unwrap()
        .arg("normalize")
        .arg(fixture_path.to_str().unwrap())
        .arg("--format=json")
        .output()
        .unwrap();

    assert!(output.status.success(), "Normalization should succeed");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Remove the log line
    let json_content = stdout
        .lines()
        .filter(|line| !line.contains("INFO"))
        .collect::<Vec<_>>()
        .join("\n");

    // For debugging
    println!("Raw JSON content:\n{}", json_content);

    // Validate structure
    validate_normalized_structure(&json_content);

    // Also verify we can parse as JSON
    let _parsed: serde_json::Value =
        serde_json::from_str(&json_content).expect("Failed to parse as JSON");

    // Snapshot testing
    insta::assert_snapshot!(json_content);
}

#[test]
fn test_normalize_invalid_file() {
    // Create a temporary directory for our invalid file
    let temp_dir = TempDir::new().unwrap();
    let invalid_file = temp_dir.path().join("invalid.rs");
    fs::write(&invalid_file, "this is not valid rust code").unwrap();

    println!("Using invalid file at: {:?}", invalid_file);

    // Build the command for debugging
    let mut cmd = Command::cargo_bin("stylusport").unwrap();
    cmd.arg("normalize")
        .arg(invalid_file.to_str().unwrap())
        .arg("--format=json");

    println!("Executing command: {:?}", cmd);

    // Execute and capture output
    let output = cmd.output().unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Adjust expectations - check for non-zero exit code instead of stderr content
    assert!(
        !output.status.success(),
        "Normalization should fail with invalid input"
    );

    // Check if there's any useful error output, either in stdout or stderr
    let error_in_stdout = !stdout.trim().is_empty();
    let error_in_stderr = !stderr.trim().is_empty();

    assert!(
        error_in_stdout || error_in_stderr,
        "Expected error output in either stdout or stderr, but both were empty"
    );
}
