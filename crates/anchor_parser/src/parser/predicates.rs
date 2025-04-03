//! Predicates for identifying Anchor-specific AST elements
//!
//! These functions help determine whether a syntax element represents
//! an Anchor-specific construct like a program module, instruction,
//! or account structure.

use syn::{ItemFn, ItemMod, ItemStruct, Type, TypePath};

/// Determines if a module is an Anchor program module
///
/// In Anchor, a program module is marked with the #[program] attribute and
/// contains instruction handlers.
///
/// # Arguments
///
/// * `module` - The module to check
///
/// # Returns
///
/// `true` if the module has the #[program] attribute
pub fn is_anchor_program(module: &ItemMod) -> bool {
    module
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("program"))
}

/// Determines if a function is an Anchor instruction handler
///
/// In Anchor, instruction handlers are functions within a #[program] module
/// that take a Context<T> parameter.
///
/// # Arguments
///
/// * `func` - The function to check
///
/// # Returns
///
/// `true` if the function takes a Context parameter
pub fn is_anchor_instruction(func: &ItemFn) -> bool {
    func.sig.inputs.iter().any(|arg| {
        if let syn::FnArg::Typed(pat_type) = arg {
            has_context_type(&pat_type.ty)
        } else {
            false
        }
    })
}

/// Determines if a type is or contains a Context
///
/// Helper function to check if a type is a Context<T> or similar.
fn has_context_type(ty: &Type) -> bool {
    match ty {
        Type::Path(path) => is_context_path(path),
        Type::Reference(reference) => has_context_type(&reference.elem),
        // Add other type variants as needed
        _ => false,
    }
}

/// Checks if a type path represents a Context
fn is_context_path(type_path: &TypePath) -> bool {
    type_path
        .path
        .segments
        .iter()
        .any(|segment| segment.ident == "Context")
}

/// Determines if a struct is an Anchor account struct
///
/// In Anchor, account structs are marked with #[derive(Accounts)] and
/// define the accounts required by an instruction.
///
/// # Arguments
///
/// * `structure` - The struct to check
///
/// # Returns
///
/// `true` if the struct has #[derive(Accounts)]
pub fn is_account_struct(structure: &ItemStruct) -> bool {
    structure.attrs.iter().any(|attr| {
        if !attr.path().is_ident("derive") {
            return false;
        }

        // Parse the derive content
        attr.parse_args_with(|content: syn::parse::ParseStream| {
            let derives = content.parse_terminated(syn::Path::parse_mod_style, syn::Token![,])?;

            // Check if Accounts is in the derive list
            for path in derives {
                if path.is_ident("Accounts") {
                    return Ok(true);
                }
            }

            Ok(false)
        })
        .unwrap_or(false)
    })
}

/// Determines if a struct is a raw Anchor account definition
///
/// In Anchor, raw account structs are marked with the #[account] attribute
/// and define the data structure stored on-chain.
///
/// # Arguments
///
/// * `structure` - The struct to check
///
/// # Returns
///
/// `true` if the struct has the #[account] attribute
pub fn is_raw_account(structure: &ItemStruct) -> bool {
    structure
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("account"))
}

#[cfg(all(test, feature = "unit_test"))]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_is_anchor_program() {
        // Module with program attribute
        let module = parse_quote! {
            #[program]
            mod my_program {}
        };
        assert!(is_anchor_program(&module));

        // Module with different attribute
        let module = parse_quote! {
            #[derive(Debug)]
            mod my_program {}
        };
        assert!(!is_anchor_program(&module));

        // Module with no attributes
        let module = parse_quote! {
            mod my_program {}
        };
        assert!(!is_anchor_program(&module));

        // Module with multiple attributes including program
        let module = parse_quote! {
            #[derive(Debug)]
            #[program]
            mod my_program {}
        };
        assert!(is_anchor_program(&module));
    }

    #[test]
    fn test_is_anchor_instruction() {
        // Function with Context parameter
        let func = parse_quote! {
            fn initialize(ctx: Context<Initialize>) {}
        };
        assert!(is_anchor_instruction(&func));

        // Function with Context parameter and other parameters
        let func = parse_quote! {
            fn initialize(ctx: Context<Initialize>, data: u64) {}
        };
        assert!(is_anchor_instruction(&func));

        // Function without Context parameter
        let func = parse_quote! {
            fn initialize(data: u64) {}
        };
        assert!(!is_anchor_instruction(&func));

        // Function with no parameters
        let func = parse_quote! {
            fn initialize() {}
        };
        assert!(!is_anchor_instruction(&func));

        // Function with reference to Context
        let func = parse_quote! {
            fn initialize(ctx: &Context<Initialize>) {}
        };
        assert!(is_anchor_instruction(&func));
    }

    #[test]
    fn test_is_account_struct() {
        // Struct with Accounts derive
        let structure = parse_quote! {
            #[derive(Accounts)]
            pub struct Initialize {}
        };
        assert!(is_account_struct(&structure));

        // Struct with different derive
        let structure = parse_quote! {
            #[derive(Debug)]
            pub struct Initialize {}
        };
        assert!(!is_account_struct(&structure));

        // Struct with multiple derives including Accounts
        let structure = parse_quote! {
            #[derive(Debug, Accounts)]
            pub struct Initialize {}
        };
        assert!(is_account_struct(&structure));

        // Struct with multiple attribute groups
        let structure = parse_quote! {
            #[derive(Debug)]
            #[derive(Accounts)]
            pub struct Initialize {}
        };
        assert!(is_account_struct(&structure));
    }

    #[test]
    fn test_is_raw_account() {
        // Struct with account attribute
        let structure = parse_quote! {
            #[account]
            pub struct Counter {}
        };
        assert!(is_raw_account(&structure));

        // Struct without account attribute
        let structure = parse_quote! {
            pub struct Counter {}
        };
        assert!(!is_raw_account(&structure));

        // Struct with account attribute and parameters
        let structure = parse_quote! {
            #[account(discriminator = [1, 2, 3, 4])]
            pub struct Counter {}
        };
        assert!(is_raw_account(&structure));
    }

    #[test]
    fn test_has_context_type() {
        // Direct Context type
        let ty = parse_quote!(Context<Initialize>);
        assert!(has_context_type(&ty));

        // Reference to Context
        let ty = parse_quote!(&Context<Initialize>);
        assert!(has_context_type(&ty));

        // Mutable reference to Context
        let ty = parse_quote!(&mut Context<Initialize>);
        assert!(has_context_type(&ty));

        // Non-Context type
        let ty = parse_quote!(u64);
        assert!(!has_context_type(&ty));
    }
}
