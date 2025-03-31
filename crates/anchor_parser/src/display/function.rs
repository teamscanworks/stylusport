//! Display functionality for Rust functions

use std::fmt::Write;
use syn::ItemFn;

/// Format a function declaration
///
/// # Arguments
///
/// * `func` - The function to format
/// * `parent_index` - The index of the parent item
/// * `item_index` - The index of this function in its parent's item list
///
/// # Returns
///
/// A formatted string representation of the function
pub fn format_function(func: &ItemFn, parent_index: usize, item_index: usize) -> String {
    let mut output = String::new();
    let indent = if parent_index > 0 && item_index > 0 {
        "      " // Nested function
    } else {
        "  " // Top-level function
    };

    writeln!(output, "{}Function name: {}", indent, func.sig.ident).unwrap();
    writeln!(output, "{}Parameters: {}", indent, func.sig.inputs.len()).unwrap();

    // Format parameter details
    for (k, input) in func.sig.inputs.iter().enumerate() {
        match input {
            syn::FnArg::Typed(pat_type) => {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    writeln!(output, "{}  Param {}: {}", indent, k + 1, pat_ident.ident).unwrap();

                    // Add parameter type information
                    write!(output, "{}    Type: ", indent).unwrap();
                    format_type(&mut output, &pat_type.ty);
                    writeln!(output).unwrap();
                }
            }
            syn::FnArg::Receiver(_) => {
                writeln!(output, "{}  Param {}: self", indent, k + 1).unwrap();
            }
        }
    }

    // Format return type if it exists
    if let syn::ReturnType::Type(_, return_type) = &func.sig.output {
        write!(output, "{}  Return type: ", indent).unwrap();
        format_type(&mut output, return_type);
        writeln!(output).unwrap();
    }

    output
}

/// Helper function to format a type
fn format_type(output: &mut String, ty: &syn::Type) {
    match ty {
        syn::Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                write!(output, "{}", segment.ident).unwrap();

                // Handle generic arguments if present
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if !args.args.is_empty() {
                        write!(output, "<...>").unwrap();
                    }
                }
            } else {
                write!(output, "unknown").unwrap();
            }
        }
        syn::Type::Reference(type_ref) => {
            write!(output, "&").unwrap();
            if type_ref.mutability.is_some() {
                write!(output, "mut ").unwrap();
            }
            format_type(output, &type_ref.elem);
        }
        _ => {
            write!(output, "complex_type").unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_str;

    #[test]
    fn test_format_function() {
        let code = "fn test_function(param1: u32, param2: &str) -> Result<(), String> {}";
        let function = parse_str::<ItemFn>(code).unwrap();

        let formatted = format_function(&function, 0, 0);

        assert!(formatted.contains("Function name: test_function"));
        assert!(formatted.contains("Parameters: 2"));
        assert!(formatted.contains("Param 1: param1"));
        assert!(formatted.contains("Param 2: param2"));
        assert!(formatted.contains("Return type: Result"));
    }
}
