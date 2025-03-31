// In display/constants.rs
//! Constants used in display formatting

/// Constants for display formatting
pub mod formatting {
    pub const INDENT_UNIT: &str = "  ";
    pub const INDENT_LEVEL_1: &str = "  ";
    pub const INDENT_LEVEL_2: &str = "    ";
    pub const INDENT_LEVEL_3: &str = "      ";

    pub const LABEL_FUNCTION: &str = "Function name:";
    pub const LABEL_MODULE: &str = "Module name:";
    pub const LABEL_STRUCT: &str = "Struct name:";
    pub const LABEL_PARAM: &str = "Param";
    pub const LABEL_FIELD: &str = "Field";
    pub const LABEL_ATTR: &str = "Attributes:";
    pub const LABEL_ITEMS: &str = "Content items:"; // This was already here
    pub const LABEL_RETURN: &str = "Return type:";
    pub const LABEL_AST: &str = "Abstract Syntax Tree (AST):";
    pub const LABEL_ITEM: &str = "Item"; // This was already here
    pub const LABEL_NESTED_MODULE: &str = "Nested module:";
    pub const LABEL_FIELDS: &str = "Fields:";
    pub const LABEL_TYPE: &str = "Type:";
    pub const LABEL_PARAMETERS: &str = "Parameters:";
}

// For identifying Anchor-specific elements
pub mod anchor {
    pub const ATTR_PROGRAM: &str = "program";
    pub const ATTR_DERIVE_ACCOUNTS: &str = "derive";
    pub const ATTR_DERIVE_ACCOUNTS_NAME: &str = "Accounts";
    pub const ATTR_ACCOUNT: &str = "account";
    pub const TYPE_CONTEXT: &str = "Context";
}
