use std::fs;
use std::path::PathBuf;
use assert_cmd::Command;
use tempfile::TempDir;

fn project_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.parent().unwrap().parent().unwrap().to_path_buf()
}

// Helper to get path to fixtures
fn fixture_path(program_name: &str) -> PathBuf {
    let mut path = project_root();
    path.push("examples/");
    path.push(program_name);
    path.push("lib.rs");
    path
}
#[test]
fn test_parse_file_yaml() {
    let fixture_path = fixture_path("hello_world");
    
    let output = Command::cargo_bin("stylusport")
        .unwrap()
        .arg("parse")
        .arg(fixture_path.to_str().unwrap())
        .arg("--format=yaml")
        .output()
        .unwrap();
    
    assert!(output.status.success(), "Parsing should succeed");
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    
    // Remove the log line
    let yaml_content = stdout.lines().filter(|line| !line.contains("INFO")).collect::<Vec<_>>().join("\n");
    
    // Debugging: Try to parse without strict validation first
    let parsed: serde_yaml::Value = serde_yaml::from_str(&yaml_content)
        .expect("Failed to parse YAML output");
    
    // Validate key structural elements
    assert!(parsed.get("program_modules").is_some(), "Missing program_modules");
    assert!(parsed.get("account_structs").is_some(), "Missing account_structs");
    
    insta::assert_snapshot!(yaml_content);
}

#[test]
fn test_parse_file_json() {
    let fixture_path = fixture_path("hello_world");
    
    let output = Command::cargo_bin("stylusport")
        .unwrap()
        .arg("parse")
        .arg(fixture_path.to_str().unwrap())
        .arg("--format=json")
        .output()
        .unwrap();
    
    assert!(output.status.success(), "Parsing should succeed");
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    
    // Remove the log line
    let json_content = stdout.lines().filter(|line| !line.contains("INFO")).collect::<Vec<_>>().join("\n");
    
    // Parse JSON
    let parsed: serde_json::Value = serde_json::from_str(&json_content)
        .expect("Failed to parse JSON output");
    
    // Validate key structural elements
    assert!(parsed.get("program_modules").is_some(), "Missing program_modules");
    assert!(parsed.get("account_structs").is_some(), "Missing account_structs");
    
    insta::assert_snapshot!(json_content);
}

#[test]
fn test_parse_invalid_file() {
    let temp_dir = TempDir::new().unwrap();
    let invalid_file = temp_dir.path().join("invalid.rs");
    fs::write(&invalid_file, "this is not valid rust code").unwrap();
    
    let output = Command::cargo_bin("stylusport")
        .unwrap()
        .arg("parse")
        .arg(invalid_file.to_str().unwrap())
        .output()
        .unwrap();
    
    // Assert command failure
    assert!(!output.status.success(), "Invalid file should cause parsing failure");
    
    // Convert stderr to string
    let stderr = String::from_utf8(output.stderr).unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    
    // More comprehensive error checking
    assert!(
        stderr.contains("Parser error") || 
        stderr.contains("Syntax error") || 
        stdout.contains("Parser error") || 
        stdout.contains("Syntax error"),
        "Error message should indicate parsing failure"
    );
}