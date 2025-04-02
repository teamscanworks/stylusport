//! Module for parsing Anchor programs into domain models
//!
//! This module provides the high-level parsing functions that convert
//! Anchor program source code into domain model objects.

use std::path::Path;
use std::fs;
use crate::error::{ParseError, Result};
use crate::model::Program;
use crate::parser::convert::convert_file;

/// Parse an Anchor program file into a Program model
///
/// # Arguments
///
/// * `file_path` - Path to the Anchor program file
///
/// # Returns
///
/// A Program model representing the Anchor program structure
///
/// # Example
///
/// ```
/// use std::path::Path;
/// use anchor_parser::parse_file;
///
/// let program = parse_file(Path::new("path/to/program.rs")).unwrap();
/// println!("Found {} program modules", program.program_modules.len());
/// ```
pub fn parse_file(file_path: &Path) -> Result<Program> {
    let file_content = fs::read_to_string(file_path)
        .map_err(|e| ParseError::Io(e))?;
    
    let ast = syn::parse_str::<syn::File>(&file_content)
        .map_err(|e| ParseError::Syntax(e))?;
    
    let mut program = convert_file(&ast)?;
    
    // Store the source path in the program
    if let Some(path_str) = file_path.to_str() {
        program = program.with_source_path(path_str);
    }
    
    Ok(program)
}

/// Parse Anchor program source code into a Program model
///
/// # Arguments
///
/// * `source` - String containing Anchor program source code
///
/// # Returns
///
/// A Program model representing the Anchor program structure
///
/// # Example
///
/// ```
/// use anchor_parser::parse_str;
///
/// let source = r#"
///     use anchor_lang::prelude::*;
///
///     #[program]
///     pub mod my_program {
///         pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
///             Ok(())
///         }
///     }
///
///     #[derive(Accounts)]
///     pub struct Initialize {}
/// "#;
///
/// let program = parse_str(source).unwrap();
/// println!("Found {} program modules", program.program_modules.len());
/// ```
pub fn parse_str(source: &str) -> Result<Program> {
    let ast = syn::parse_str::<syn::File>(source)
        .map_err(|e| ParseError::Syntax(e))?;
    
    convert_file(&ast)
}

#[cfg(all(test, feature = "unit_test"))]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    // Helper to create and parse a temporary file
    fn parse_temp_code(code: &str) -> Result<Program> {
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", code).unwrap();
        temp_file.flush().unwrap();
        parse_file(temp_file.path())
    }

    #[test]
    fn test_parse_valid_anchor_program() {
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

        // Test parsing into our domain model
        let result = parse_str(valid_program);
        assert!(result.is_ok(), "Failed to parse valid Anchor program");

        let program = result.unwrap();
        
        // Verify program structure
        assert_eq!(program.program_modules.len(), 1, "Should have one program module");
        assert_eq!(program.program_modules[0].name, "my_program");
        assert_eq!(program.program_modules[0].instructions.len(), 1);
        assert_eq!(program.program_modules[0].instructions[0].name, "initialize");
        
        assert_eq!(program.account_structs.len(), 1, "Should have one account struct");
        assert_eq!(program.account_structs[0].name, "Initialize");
    }

    #[test]
    fn test_parse_invalid_syntax() {
        let invalid_program = r#"
        use anchor_lang::prelude::*;

        #[program
        pub mod my_program {
            // Missing closing bracket
        "#;

        // Test error handling with our domain-specific error type
        let result = parse_str(invalid_program);
        assert!(result.is_err(), "Should fail to parse invalid Rust syntax");
        
        match result {
            Err(ParseError::Syntax(_)) => {}, // Expected error type
            _ => panic!("Expected a syntax error"),
        }
    }

    #[test]
    fn test_parse_non_anchor_program() {
        // Test parsing a regular Rust file with no Anchor attributes
        let non_anchor = r#"
        fn regular_function() -> u32 {
            42
        }
        "#;
        
        let result = parse_str(non_anchor);
        assert!(result.is_ok(), "Should parse non-Anchor code without errors");
        
        let program = result.unwrap();
        assert!(program.program_modules.is_empty(), "Should have no program modules");
        assert!(program.account_structs.is_empty(), "Should have no account structs");
    }
    
    #[test]
    fn test_parse_anchor_with_multiple_modules() {
        let complex_program = r#"
        use anchor_lang::prelude::*;

        #[program]
        pub mod token_program {
            pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
                Ok(())
            }
            
            pub fn mint(_ctx: Context<Mint>, amount: u64) -> Result<()> {
                Ok(())
            }
        }
        
        #[program]
        mod admin_program {
            pub fn configure(_ctx: Context<Configure>) -> Result<()> {
                Ok(())
            }
        }

        #[derive(Accounts)]
        pub struct Initialize {}
        
        #[derive(Accounts)]
        pub struct Mint {
            #[account(signer)]
            pub authority: AccountInfo<'info>,
            #[account(mut)]
            pub token_account: Account<'info, TokenAccount>,
        }
        
        #[derive(Accounts)]
        pub struct Configure {}
        
        #[account]
        pub struct TokenAccount {
            pub owner: Pubkey,
            pub amount: u64,
        }
        "#;

        let result = parse_str(complex_program);
        assert!(result.is_ok(), "Failed to parse complex Anchor program");

        let program = result.unwrap();
        
        // Verify program structure
        assert_eq!(program.program_modules.len(), 2, "Should have two program modules");
        assert_eq!(program.account_structs.len(), 3, "Should have three account structs");
        assert_eq!(program.raw_accounts.len(), 1, "Should have one raw account");
        
        // Verify first program module
        let token_module = program.find_program_module("token_program").unwrap();
        assert_eq!(token_module.instructions.len(), 2);
        
        // Verify second program module
        let admin_module = program.find_program_module("admin_program").unwrap();
        assert_eq!(admin_module.instructions.len(), 1);
        
        // Verify account structs
        let mint_account = program.find_account_struct("Mint").unwrap();
        assert_eq!(mint_account.fields.len(), 2);
        
        // Verify raw accounts
        let token_account = program.find_raw_account("TokenAccount").unwrap();
        assert_eq!(token_account.fields.len(), 2);
    }
}