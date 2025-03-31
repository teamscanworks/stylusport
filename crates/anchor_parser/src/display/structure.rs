//! Display functionality for Rust structs

use std::fmt::Write;
use syn::ItemStruct;

/// Format a struct declaration
///
/// # Arguments
///
/// * `structure` - The struct to format
/// * `parent_index` - The index of the parent item
/// * `item_index` - The index of this struct in its parent's item list
///
/// # Returns
///
/// A formatted string representation of the struct
pub fn format_struct(structure: &ItemStruct, parent_index: usize, item_index: usize) -> String {
    let mut output = String::new();
    let indent = if parent_index > 0 && item_index > 0 {
        "      " // Nested struct
    } else {
        "  " // Top-level struct
    };

    writeln!(output, "{}Struct name: {}", indent, structure.ident).unwrap();

    // Count and display fields
    let field_count = count_fields(structure);
    writeln!(output, "{}Fields: {}", indent, field_count).unwrap();

    // Format fields if present
    match &structure.fields {
        syn::Fields::Named(fields) => {
            for (i, field) in fields.named.iter().enumerate() {
                if let Some(ident) = &field.ident {
                    writeln!(output, "{}  Field {}: {}", indent, i + 1, ident).unwrap();

                    // Add field type information
                    write!(output, "{}    Type: ", indent).unwrap();
                    format_type(&mut output, &field.ty);
                    writeln!(output).unwrap();
                }
            }
        }
        syn::Fields::Unnamed(fields) => {
            for (i, field) in fields.unnamed.iter().enumerate() {
                writeln!(output, "{}  Field {}: (unnamed)", indent, i + 1).unwrap();

                // Add field type information
                write!(output, "{}    Type: ", indent).unwrap();
                format_type(&mut output, &field.ty);
                writeln!(output).unwrap();
            }
        }
        syn::Fields::Unit => {}
    }

    output
}

/// Count fields in a struct
fn count_fields(structure: &ItemStruct) -> usize {
    match &structure.fields {
        syn::Fields::Named(fields) => fields.named.len(),
        syn::Fields::Unnamed(fields) => fields.unnamed.len(),
        syn::Fields::Unit => 0,
    }
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
    fn test_format_struct() {
        let code = "struct TestStruct { field1: u32, field2: String }";
        let structure = parse_str::<ItemStruct>(code).unwrap();

        let formatted = format_struct(&structure, 0, 0);

        assert!(formatted.contains("Struct name: TestStruct"));
        assert!(formatted.contains("Fields: 2"));
        assert!(formatted.contains("Field 1: field1"));
        assert!(formatted.contains("Field 2: field2"));
    }
}
