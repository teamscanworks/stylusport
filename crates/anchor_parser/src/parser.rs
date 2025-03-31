//! Module for parsing Anchor programs into AST

use std::error::Error;
use std::fs;
use std::path::Path;
use syn::File;

/// Parse an Anchor program file into its full AST
///
/// # Arguments
///
/// * `file_path` - Path to the Anchor program file
///
/// # Returns
///
/// The parsed syntax tree or an error
pub fn parse_file(file_path: &Path) -> Result<File, Box<dyn Error>> {
    let file_content = fs::read_to_string(file_path)?;
    parse_str(&file_content) // Reuse parse_str for consistency
}

// String-based parsing (available for testing and regular use)
pub fn parse_str(code: &str) -> Result<File, Box<dyn Error>> {
    match syn::parse_str::<File>(code) {
        Ok(file) => Ok(file),
        Err(e) => Err(Box::new(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    // Helper to create and parse a temporary file
    fn parse_temp_code(code: &str) -> Result<File, Box<dyn Error>> {
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", code).unwrap();
        temp_file.flush().unwrap();
        parse_file(temp_file.path())
    }

    #[test]
    fn test_parse_valid_anchor_file() {
        let valid_program = r#"
        use anchor_lang::prelude::*;

        #[program]
        pub mod my_program {
            pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
                Ok(())
            }
        }

        #[derive(Accounts)]
        pub struct Initialize {}
        "#;

        // Test basic parsing success
        let result = parse_temp_code(valid_program);
        assert!(result.is_ok(), "Failed to parse valid Anchor program");

        // Verify we have the expected number of items
        let ast = result.unwrap();
        assert!(
            ast.items.len() >= 3,
            "Expected at least 3 items (use, mod, struct)"
        );
    }

    #[test]
    fn test_parse_invalid_syntax() {
        let invalid_program = r#"
        use anchor_lang::prelude::*;

        #[program
        pub mod my_program {
            // Missing closing bracket
        "#;

        // Test error handling
        let result = parse_temp_code(invalid_program);
        assert!(result.is_err(), "Should fail to parse invalid Rust syntax");
    }

    #[test]
    fn test_parse_str() {
        // Test simple string parsing
        let code = "fn simple_function() -> u32 { 42 }";
        let result = parse_str(code);

        assert!(result.is_ok(), "Failed to parse simple function");
        let ast = result.unwrap();
        assert_eq!(ast.items.len(), 1, "Should have exactly one function");
    }
}
