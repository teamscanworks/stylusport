//! Formatting utilities for text output

/// Formats an indent string based on nesting level
///
/// # Arguments
///
/// * `level` - The indentation level (0 = no indent, 1 = base indent, etc.)
///
/// # Returns
///
/// A string with the appropriate number of spaces for indentation
pub fn indent(level: usize) -> &'static str {
    match level {
        0 => "",
        1 => "  ",
        2 => "    ",
        3 => "      ",
        4 => "        ",
        _ => "          ", // Max indent level for readability
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indent_various_levels() {
        assert_eq!(indent(0), "");
        assert_eq!(indent(1), "  ");
        assert_eq!(indent(2), "    ");
        assert_eq!(indent(3), "      ");
        assert_eq!(indent(4), "        ");
        assert_eq!(indent(5), "          "); // Maximum level
    }
}
