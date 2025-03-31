//! Utility functions for AST display

use syn::Item;

/// Helper function to get a readable name for item types
///
/// # Arguments
///
/// * `item` - The AST item to get a name for
///
/// # Returns
///
/// A human-readable string representation of the item type
pub fn item_type_name(item: &Item) -> &'static str {
    match item {
        Item::Const(_) => "Constant",
        Item::Enum(_) => "Enum",
        Item::ExternCrate(_) => "Extern Crate",
        Item::Fn(_) => "Function",
        Item::ForeignMod(_) => "Foreign Module",
        Item::Impl(_) => "Implementation",
        Item::Macro(_) => "Macro",
        Item::Mod(_) => "Module",
        Item::Static(_) => "Static",
        Item::Struct(_) => "Struct",
        Item::Trait(_) => "Trait",
        Item::TraitAlias(_) => "Trait Alias",
        Item::Type(_) => "Type",
        Item::Union(_) => "Union",
        Item::Use(_) => "Use",
        Item::Verbatim(_) => "Verbatim",
        _ => "Unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_str;
    use utils::fmt::indent;

    /// Provides a simplified string representation of a path
    ///
    /// # Arguments
    ///
    /// * `path` - The syntax path to simplify
    ///
    /// # Returns
    ///
    /// A string representation of the path
    pub fn format_path(path: &syn::Path) -> String {
        path.segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>()
            .join("::")
    }

    #[test]
    fn test_item_type_name_all_variants() {
        // Test all possible item types
        let cases = [
            ("const VALUE: u32 = 42;", "Constant"),
            ("enum TestEnum { A, B }", "Enum"),
            ("extern crate test;", "Extern Crate"),
            ("fn test_fn() {}", "Function"),
            ("extern \"C\" { fn test(); }", "Foreign Module"),
            ("impl Test { fn method() {} }", "Implementation"),
            ("macro_rules! test { () => {}; }", "Macro"),
            ("mod test_mod {}", "Module"),
            ("static VALUE: u32 = 42;", "Static"),
            ("struct TestStruct {}", "Struct"),
            ("trait TestTrait {}", "Trait"),
            ("type TestType = u32;", "Type"),
            ("union TestUnion { a: u32, b: f32 }", "Union"),
            ("use std::io;", "Use"),
            // Verbatim is for custom syntax and hard to test with parse_str
        ];

        for (code, expected_name) in cases {
            if let Ok(item) = parse_str::<Item>(code) {
                assert_eq!(item_type_name(&item), expected_name);
            }
        }
    }

    #[test]
    fn test_indent_various_levels() {
        assert_eq!(indent(0), "");
        assert_eq!(indent(1), "  ");
        assert_eq!(indent(2), "    ");
        assert_eq!(indent(3), "      ");
        assert_eq!(indent(4), "        ");
        assert_eq!(indent(5), "          "); // Maximum level
    }

    #[test]
    fn test_format_path() {
        // Simple path
        let path: syn::Path = parse_str("std::io::Read").unwrap();
        assert_eq!(format_path(&path), "std::io::Read");

        // Single segment path
        let path: syn::Path = parse_str("usize").unwrap();
        assert_eq!(format_path(&path), "usize");

        // Path with generic arguments (they should be ignored)
        let path: syn::Path = parse_str("Option<T>").unwrap();
        assert_eq!(format_path(&path), "Option");
    }
}
