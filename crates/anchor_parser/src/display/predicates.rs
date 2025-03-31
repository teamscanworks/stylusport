/// Determines if a module is an Anchor program
///
/// # Arguments
///
/// * `module` - The module to check
///
/// # Returns
///
/// `true` if the module has the #[program] attribute
pub fn is_anchor_program(module: &syn::ItemMod) -> bool {
    module.attrs.iter().any(|attr| {
        attr.path().get_ident()
            .map_or(false, |ident| ident == "program")
    })
}

/// Determines if a function is an Anchor instruction
///
/// # Arguments
///
/// * `func` - The function to check
///
/// # Returns
///
/// `true` if the function takes a Context parameter
pub fn is_anchor_instruction(func: &syn::ItemFn) -> bool {
    func.sig.inputs.iter().any(|arg| {
        if let syn::FnArg::Typed(pat_type) = arg {
            if let syn::Type::Path(type_path) = &*pat_type.ty {
                type_path.path.segments.iter().any(|segment| 
                    segment.ident == "Context"
                )
            } else {
                false
            }
        } else {
            false
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_str;
    
    #[test]
    fn test_is_anchor_program() {
        // Module with program attribute
        let module: syn::ItemMod = parse_str("#[program] mod my_program {}").unwrap();
        assert!(is_anchor_program(&module));
        
        // Module with different attribute
        let module: syn::ItemMod = parse_str("#[derive(Debug)] mod my_program {}").unwrap();
        assert!(!is_anchor_program(&module));
        
        // Module with no attributes
        let module: syn::ItemMod = parse_str("mod my_program {}").unwrap();
        assert!(!is_anchor_program(&module));
        
        // Module with multiple attributes including program
        let module: syn::ItemMod = parse_str("#[derive(Debug)] #[program] mod my_program {}").unwrap();
        assert!(is_anchor_program(&module));
    }
    
    #[test]
    fn test_is_anchor_instruction() {
        // Function with Context parameter
        let func: syn::ItemFn = parse_str("fn initialize(ctx: Context<Initialize>) {}").unwrap();
        assert!(is_anchor_instruction(&func));
        
        // Function with Context parameter and other parameters
        let func: syn::ItemFn = parse_str("fn initialize(ctx: Context<Initialize>, data: u64) {}").unwrap();
        assert!(is_anchor_instruction(&func));
        
        // Function without Context parameter
        let func: syn::ItemFn = parse_str("fn initialize(data: u64) {}").unwrap();
        assert!(!is_anchor_instruction(&func));
        
        // Function with no parameters
        let func: syn::ItemFn = parse_str("fn initialize() {}").unwrap();
        assert!(!is_anchor_instruction(&func));
        
        // Function with a parameter that contains "Context" but is not a Context
        let func: syn::ItemFn = parse_str("fn initialize(ctx: MyContext) {}").unwrap();
        assert!(!is_anchor_instruction(&func));
        
        // Method with self parameter and Context
        let func: syn::ItemFn = parse_str("fn initialize(&self, ctx: Context<Initialize>) {}").unwrap();
        assert!(is_anchor_instruction(&func));
    }

}