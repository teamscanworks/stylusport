// File: anchor_parser/tests/parse_fixtures.rs

use anchor_parser::display::formatting::strings::{
    format_function_name, format_module_name, format_struct_name,
};
use anchor_parser::{display::format_ast, parse_file};
use std::path::PathBuf;

// Helper to get path to fixtures
fn fixture_path(program_name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/fixtures");
    path.push(program_name);
    path.push("lib.rs");
    path
}

// Generic test function that can be reused for different programs
fn test_parse_program(program_name: &str, expected_elements: Vec<&str>) {
    // Parse the program
    let fixture = fixture_path(program_name);
    let ast_result = parse_file(&fixture);

    // Handle parsing errors with informative output
    if let Err(err) = ast_result {
        println!("==== ERROR PARSING {} ====", program_name);
        println!("{}", err);
        panic!("Failed to parse {} fixture", program_name);
    }

    let ast = ast_result.unwrap();

    // Format the AST
    let formatted = format_ast(&ast);

    // Only check each expected element
    let mut missing_elements = Vec::new();
    for element in &expected_elements {
        if !formatted.contains(element) {
            missing_elements.push(*element);
        }
    }

    // If elements are missing, display the AST for troubleshooting
    if !missing_elements.is_empty() {
        println!(
            "\n==== FORMATTED AST FOR {} ====",
            program_name.to_uppercase()
        );
        println!("{}", formatted);
        println!("===============================\n");

        println!("==== MISSING ELEMENTS ====");
        for element in missing_elements {
            println!("- \"{}\"", element);
        }
        println!("=========================\n");

        panic!("Expected elements not found in {} AST", program_name);
    }
}

#[test]
fn test_hello_world() {
    test_parse_program(
        "hello_world",
        vec![
            &format_module_name("hello_world"),
            &format_function_name("initialize"),
            &format_struct_name("Initialize"),
        ],
    );
}

#[test]
fn test_counter() {
    test_parse_program(
        "counter",
        vec![
            &format_module_name("counter"),
            &format_function_name("initialize"),
            &format_function_name("increment"),
            &format_struct_name("Counter"),
            &format_struct_name("Initialize"),
            &format_struct_name("Increment"),
        ],
    );
}

#[test]
fn test_token_vault() {
    test_parse_program(
        "token_vault",
        vec![
            &format_module_name("token_vault"),
            &format_function_name("initialize"),
            &format_function_name("deposit"),
            &format_struct_name("Vault"),
            &format_struct_name("Initialize"),
            &format_struct_name("Deposit"),
        ],
    );
}
