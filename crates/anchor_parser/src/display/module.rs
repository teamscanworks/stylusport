//! Display functionality for Rust modules

//! Display functionality for Rust modules

use super::formatting::strings::{
    format_attributes_count, format_item_reference, format_items_count, format_module_name,
};
use super::function::format_function;
use super::structure::format_struct;
use super::utils::item_type_name;
use std::fmt::Write;
use syn::{Item, ItemMod};
/// Format a module item
///
/// # Arguments
///
/// * `module` - The module to format
/// * `index` - The index of this module in its parent list
///
/// # Returns
///
/// A formatted string representation of the module
pub fn format_module(module: &ItemMod, index: usize) -> String {
    let mut output = String::new();

    // Use string formatting helpers for simple text elements
    writeln!(output, "{}", format_module_name(&module.ident.to_string())).unwrap();
    writeln!(output, "  {}", format_attributes_count(module.attrs.len())).unwrap();

    // Format attributes
    for attr in &module.attrs {
        if let Some(ident) = attr.path().get_ident() {
            writeln!(output, "    - {}", ident).unwrap();
        }
    }

    // Format module content if available
    if let Some((_, items)) = &module.content {
        writeln!(output, "  {}", format_items_count(items.len())).unwrap();
        for (j, inner_item) in items.iter().enumerate() {
            writeln!(
                output,
                "    {}",
                format_item_reference(index, j + 1, item_type_name(inner_item))
            )
            .unwrap();

            // Use component formatting functions for complete AST nodes
            match inner_item {
                Item::Fn(func) => {
                    write!(output, "{}", format_function(func, index, j + 1)).unwrap();
                }
                Item::Struct(structure) => {
                    write!(output, "{}", format_struct(structure, index, j + 1)).unwrap();
                }
                Item::Mod(nested_module) => {
                    // Use a different indentation level for nested modules
                    let nested_str = format_nested_module(nested_module, index, j + 1);
                    write!(output, "{}", nested_str).unwrap();
                }
                _ => {}
            }
        }
    }

    output
}

/// Format a nested module with proper indentation
fn format_nested_module(module: &ItemMod, _parent_index: usize, _item_index: usize) -> String {
    let mut output = String::new();

    writeln!(output, "      Nested module: {}", module.ident).unwrap();

    // We could add more detailed nested module formatting here

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_str;

    #[test]
    fn test_format_module() {
        let code = "pub mod test_module { fn inner_fn() {} }";
        let module = parse_str::<ItemMod>(code).unwrap();

        let formatted = format_module(&module, 1);

        assert!(formatted.contains(&format_module_name("test_module")));
        assert!(formatted.contains(&format_items_count(1)));
    }
}
