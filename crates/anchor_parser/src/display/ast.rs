//! Display functionality for the top-level AST

use super::function::format_function;
use super::module::format_module;
use super::structure::format_struct;
use super::utils::item_type_name;
use std::fmt::Write;
use syn::File; // Change to struct_
/// Format the full AST structure as a string
///
/// # Arguments
///
/// * `file` - The parsed syntax tree
///
/// # Returns
///
/// A formatted string representation of the AST
pub fn format_ast(file: &File) -> String {
    let mut output = String::new();

    writeln!(output, "Abstract Syntax Tree (AST):").unwrap();

    for (i, item) in file.items.iter().enumerate() {
        writeln!(output, "Item {}: {}", i + 1, item_type_name(item)).unwrap();

        match item {
            syn::Item::Mod(module) => {
                write!(output, "{}", format_module(module, i + 1)).unwrap();
            }
            syn::Item::Fn(func) => {
                write!(output, "{}", format_function(func, i + 1, 1)).unwrap();
            }
            syn::Item::Struct(structure) => {
                write!(output, "{}", format_struct(structure, i + 1, 1)).unwrap();
            }
            _ => {}
        }

        writeln!(output).unwrap();
    }

    output
}

/// Print the AST to standard output
///
/// # Arguments
///
/// * `file` - The parsed syntax tree
pub fn print_ast(file: &File) {
    print!("{}", format_ast(file));
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::display::formatting::strings::{format_module_name, format_struct_name};
    use syn::parse_str;

    #[test]
    fn test_format_ast() {
        let code = r#"
        use anchor_lang::prelude::*;

        #[program]
        pub mod test_program {
            fn test_function() {}
        }

        #[derive(Accounts)]
        pub struct TestAccounts {
            #[account(mut)]
            pub user: Signer<'info>,
            pub system_program: Program<'info, System>,
        }
        "#;

        let file = parse_str::<File>(code).unwrap();
        let formatted = format_ast(&file);
        println!("{}", formatted);

        // Basic structural checks using helper functions
        assert!(formatted.contains("Abstract Syntax Tree"));
        assert!(formatted.contains(&format_module_name("test_program")));

        // Check struct formatting using helper functions
        assert!(formatted.contains(&format_struct_name("TestAccounts")));

        // For fields, we need to add those helper functions or use literals for now
        assert!(formatted.contains("Fields: 2"));
        assert!(formatted.contains("Field 1: user"));
        assert!(formatted.contains("Field 2: system_program"));
        assert!(formatted.contains("Type: Signer"));
        assert!(formatted.contains("Type: Program"));
    }
}
