//! Conversion from Rust syntax tree to Anchor program model
//!
//! This module is responsible for traversing the Rust syntax tree produced by `syn`
//! and converting it into our domain-specific model of Anchor programs.

use crate::error::{ParseError, Result};
use crate::model::{
    Account, AccountField, Constraint, Instruction, Parameter, 
    Program, ProgramModule, RawAccount, RawAccountField
};
use crate::parser::predicates;
use syn::{Attribute, File, Item, ItemFn, ItemStruct, Visibility};
use quote::ToTokens;

/// Convert a parsed syntax tree to our Program model
///
/// This is the main entry point for syntax conversion and is called by the parse functions
/// after they've parsed the source code into a syntax tree.
///
/// # Arguments
///
/// * `file` - The parsed syntax tree
///
/// # Returns
///
/// A Program model representing the Anchor program
pub fn convert_file(file: &File) -> Result<Program> {
    let mut program = Program::new();
    
    // Process each item in the file
    for item in &file.items {
        process_item(&mut program, item)?;
    }
    
    Ok(program)
}

/// Process a top-level syntax item
fn process_item(program: &mut Program, item: &Item) -> Result<()> {
    match item {
        Item::Mod(module) => {
            if predicates::is_anchor_program(module) {
                // Found a program module
                let module_name = module.ident.to_string();
                let visibility = format_visibility(&module.vis);
                
                let mut program_module = ProgramModule::new(module_name, visibility);
                
                // Process its contents if available
                if let Some((_, items)) = &module.content {
                    for item in items {
                        process_program_item(&mut program_module, item)?;
                    }
                }
                
                program.add_program_module(program_module);
            }
        },
        Item::Struct(structure) => {
            if predicates::is_account_struct(structure) {
                // Convert to our Account model
                let account = convert_account_struct(structure)?;
                program.add_account_struct(account);
            } else if predicates::is_raw_account(structure) {
                // Convert to our RawAccount model
                let raw_account = convert_raw_account(structure)?;
                program.add_raw_account(raw_account);
            }
        },
        // Other items can be ignored or processed as needed
        _ => {}
    }
    
    Ok(())
}

/// Process an item within a program module
fn process_program_item(program_module: &mut ProgramModule, item: &Item) -> Result<()> {
    match item {
        Item::Fn(function) => {
            if predicates::is_anchor_instruction(function) {
                // Convert to our Instruction model
                let instruction = convert_instruction(function)?;
                program_module.add_instruction(instruction);
            }
        },
        // Other items can be ignored or processed as needed
        _ => {}
    }
    
    Ok(())
}

/// Convert a syn ItemStruct to our Account model
fn convert_account_struct(structure: &ItemStruct) -> Result<Account> {
    let name = structure.ident.to_string();
    let visibility = format_visibility(&structure.vis);
    
    let mut account = Account::new(name, visibility);
    
    // Process fields
    for field in &structure.fields {
        if let Some(ident) = &field.ident {
            let field_name = ident.to_string();
            let field_type = format_type(&field.ty);
            
            let mut account_field = AccountField::new(field_name, field_type);
            
            // Process account attribute constraints
            for attr in &field.attrs {
                if attr.path().is_ident("account") {
                    process_account_constraints(attr, &mut account_field)?;
                }
            }
            
            account.add_field(account_field);
        }
    }
    
    Ok(account)
}
/// Process the constraints in an #[account(...)] attribute
fn process_account_constraints(attr: &Attribute, field: &mut AccountField) -> Result<()> {
    // Get the attribute content as a string
    let attr_str = attr.to_token_stream().to_string();
    
    // Extract contents between parentheses
    if let Some(start) = attr_str.find('(') {
        if let Some(end) = attr_str.rfind(')') {
            let content = &attr_str[start + 1..end];
            
            // Parse the content manually
            let mut constraints = Vec::new();
            let mut current = String::new();
            let mut depth = 0;
            
            for c in content.chars() {
                match c {
                    '(' | '[' | '{' => {
                        depth += 1;
                        current.push(c);
                    },
                    ')' | ']' | '}' => {
                        depth -= 1;
                        current.push(c);
                    },
                    ',' if depth == 0 => {
                        if !current.trim().is_empty() {
                            constraints.push(current.trim().to_string());
                            current.clear();
                        }
                    },
                    _ => current.push(c),
                }
            }
            
            if !current.trim().is_empty() {
                constraints.push(current.trim().to_string());
            }
            
            // Process each constraint
            for constraint in constraints {
                if let Some(idx) = constraint.find('=') {
                    let name = constraint[..idx].trim().to_string();
                    let value = constraint[idx+1..].trim().to_string();
                    field.add_constraint(Constraint::with_value(name, value));
                } else {
                    field.add_constraint(Constraint::without_value(constraint));
                }
            }
            
            return Ok(());
        }
    }
    
    Err(ParseError::Parse("Failed to parse account attribute".to_string()))
}
/// Convert a syn ItemStruct to our RawAccount model
fn convert_raw_account(structure: &ItemStruct) -> Result<RawAccount> {
    let name = structure.ident.to_string();
    let visibility = format_visibility(&structure.vis);
    
    let mut raw_account = RawAccount::new(name, visibility);
    
    // Process fields
    for field in &structure.fields {
        if let Some(ident) = &field.ident {
            let field_name = ident.to_string();
            let field_type = format_type(&field.ty);
            let field_vis = format_visibility(&field.vis);
            
            let raw_field = RawAccountField::new(field_name, field_type, field_vis);
            raw_account.add_field(raw_field);
        }
    }
    
    Ok(raw_account)
}

/// Convert a syn ItemFn to our Instruction model
fn convert_instruction(function: &ItemFn) -> Result<Instruction> {
    let name = function.sig.ident.to_string();
    let visibility = format_visibility(&function.vis);
    
    let mut instruction = Instruction::new(name, visibility);
    
    // Set return type if available
    if let syn::ReturnType::Type(_, ty) = &function.sig.output {
        instruction.set_return_type(format_type(ty));
    }
    
    // Process parameters
    for input in &function.sig.inputs {
        match input {
            syn::FnArg::Typed(pat_type) => {
                // Get parameter name
                let param_name = match &*pat_type.pat {
                    syn::Pat::Ident(ident) => ident.ident.to_string(),
                    _ => "unnamed".to_string(),
                };
                
                // Check if this is a Context parameter
                let (is_context, context_type) = get_context_info(&pat_type.ty);
                
                let param_type = format_type(&pat_type.ty);
                
                // If it's a Context, set the context type
                if is_context {
                    if let Some(ctx_type) = context_type {
                        instruction.set_context_type(ctx_type);
                    }
                }
                
                let parameter = Parameter::new(param_name, param_type, is_context);
                instruction.add_parameter(parameter);
            },
            _ => {},
        }
    }
    
    Ok(instruction)
}

/// Analyze a type to determine if it's a Context type and extract its generic parameter
fn get_context_info(ty: &syn::Type) -> (bool, Option<String>) {
    if let syn::Type::Path(type_path) = ty {
        if type_path.path.segments.iter().any(|segment| segment.ident == "Context") {
            // It's a Context, now extract the generic type
            if let Some(segment) = type_path.path.segments.last() {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(arg) = args.args.first() {
                        if let syn::GenericArgument::Type(inner_ty) = arg {
                            return (true, Some(format_type(inner_ty)));
                        }
                    }
                }
            }
            return (true, None);
        }
    }
    (false, None)
}

/// Format a visibility to a string
fn format_visibility(vis: &Visibility) -> String {
    match vis {
        Visibility::Public(_) => "pub".to_string(),
        Visibility::Restricted(restricted) => {
            format!("pub({})", restricted.path.to_token_stream())
        },
        Visibility::Inherited => "".to_string(),
    }
}

/// Format a type to a string
fn format_type(ty: &syn::Type) -> String {
    let raw = ty.to_token_stream().to_string();
    
    // First, normalize spaces around punctuation
    let intermediate = raw
        .replace(" : ", ":")
        .replace(": ", ":")
        .replace(" :", ":")
        
        .replace(" < ", "<")
        .replace("< ", "<")
        .replace(" <", "<")
        
        .replace(" > ", ">")
        .replace("> ", ">")
        .replace(" >", ">")
        
        .replace(" ( ", "(")
        .replace("( ", "(")
        .replace(" (", "(")
        
        .replace(" ) ", ")")
        .replace(") ", ")")
        .replace(" )", ")")
        
        .replace(" [ ", "[")
        .replace("[ ", "[")
        .replace(" [", "[")
        
        .replace(" ] ", "]")
        .replace("] ", "]")
        .replace(" ]", "]")
        
        .replace(" , ", ",")
        .replace(", ", ",")
        .replace(" ,", ",");
    
    // Normalize any remaining multiple spaces to single spaces
    let result = intermediate.split_whitespace().collect::<Vec<_>>().join(" ");
    
    result
}

#[cfg(all(test, feature = "unit_test"))]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_convert_program_module() {
        // Create a program module with syn
        let module = parse_quote! {
            #[program]
            pub mod my_program {
                pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
                    Ok(())
                }
                
                pub fn update(ctx: Context<Update>, value: u64) -> Result<()> {
                    Ok(())
                }
            }
        };
        
        // Create a program to hold it
        let mut program = Program::new();
        
        // Process the module
        process_item(&mut program, &Item::Mod(module)).unwrap();
        
        // Verify the result
        assert_eq!(program.program_modules.len(), 1);
        let program_module = &program.program_modules[0];
        
        assert_eq!(program_module.name, "my_program");
        assert_eq!(program_module.visibility, "pub");
        assert_eq!(program_module.instructions.len(), 2);
        
        // Verify the first instruction
        let init_instr = &program_module.instructions[0];
        assert_eq!(init_instr.name, "initialize");
        assert_eq!(init_instr.visibility, "pub");
        assert_eq!(init_instr.parameters.len(), 1);
        assert_eq!(init_instr.parameters[0].name, "ctx");
        assert!(init_instr.parameters[0].is_context);
        assert_eq!(init_instr.context_type, Some("Initialize".to_string()));
        
        // Verify the second instruction
        let update_instr = &program_module.instructions[1];
        assert_eq!(update_instr.name, "update");
        assert_eq!(update_instr.parameters.len(), 2);
        assert_eq!(update_instr.parameters[1].name, "value");
        assert_eq!(update_instr.parameters[1].ty, "u64");
        assert!(!update_instr.parameters[1].is_context);
    }

    #[test]
    fn test_convert_account_struct() {
        // Create an account struct with syn
        let account_struct = parse_quote! {
            #[derive(Accounts)]
            pub struct Initialize {
                #[account(signer)]
                pub user: AccountInfo<'info>,
                
                #[account(init, payer = user)]
                pub data: Account<'info, UserData>,
                
                pub system_program: Program<'info, System>,
            }
        };
        
        // Convert it
        let account = convert_account_struct(&account_struct).unwrap();
        
        // Verify the result
        assert_eq!(account.name, "Initialize");
        assert_eq!(account.visibility, "pub");
        assert_eq!(account.fields.len(), 3);
        
        // Check the first field
        let user_field = account.find_field("user").unwrap();
        assert_eq!(user_field.name, "user");
        assert!(user_field.constraints.iter().any(|c| c.constraint_type == "signer"));
        
        // Check the second field
        let data_field = account.find_field("data").unwrap();
        assert_eq!(data_field.name, "data");
        assert!(data_field.constraints.iter().any(|c| c.constraint_type == "init"));
        assert!(data_field.constraints.iter().any(|c| c.constraint_type == "payer"));
    }

    #[test]
    fn test_convert_raw_account() {
        // Create a raw account struct with syn
        let raw_account_struct = parse_quote! {
            #[account]
            pub struct UserData {
                pub owner: Pubkey,
                pub balance: u64,
                created_at: i64,
            }
        };
        
        // Convert it
        let raw_account = convert_raw_account(&raw_account_struct).unwrap();
        
        // Verify the result
        assert_eq!(raw_account.name, "UserData");
        assert_eq!(raw_account.visibility, "pub");
        assert_eq!(raw_account.fields.len(), 3);
        
        // Check fields
        let owner_field = raw_account.find_field("owner").unwrap();
        assert_eq!(owner_field.name, "owner");
        assert_eq!(owner_field.visibility, "pub");
        assert_eq!(owner_field.ty, "Pubkey");
        
        let created_field = raw_account.find_field("created_at").unwrap();
        assert_eq!(created_field.name, "created_at");
        assert_eq!(created_field.visibility, ""); // Not public
    }

    #[test]
    fn test_format_visibility() {
        let public: Visibility = parse_quote!(pub);
        assert_eq!(format_visibility(&public), "pub");
        
        let restricted: Visibility = parse_quote!(pub(crate));
        assert_eq!(format_visibility(&restricted), "pub(crate)");
        
        let private: Visibility = parse_quote!();
        assert_eq!(format_visibility(&private), "");
    }

    #[test]
    fn test_format_type() {
        let simple_type: syn::Type = parse_quote!(u64);
        assert_eq!(format_type(&simple_type), "u64");
        
        let generic_type: syn::Type = parse_quote!(Option<String>);
        assert_eq!(format_type(&generic_type), "Option<String>");
        
        let complex_type: syn::Type = parse_quote!(HashMap<Pubkey, Vec<u8>>);
        assert_eq!(format_type(&complex_type), "HashMap<Pubkey,Vec<u8>>");
    }

    #[test]
    fn test_convert_account_struct_with_mut() {
        // Create an account struct with keyword constraints (mut, etc)
        let account_struct = parse_quote! {
            #[derive(Accounts)]
            pub struct Transfer {
                #[account(mut, signer)]
                pub authority: Signer<'info>,
                
                #[account(mut)]
                pub from: Account<'info, TokenAccount>,
                
                pub to: Account<'info, TokenAccount>,
            }
        };
        
        // Convert it
        let account = convert_account_struct(&account_struct).unwrap();
        
        // Verify the result
        assert_eq!(account.name, "Transfer");
        
        // Check that mut was parsed correctly for authority
        let authority_field = account.find_field("authority").unwrap();
        assert!(authority_field.constraints.iter().any(|c| c.constraint_type == "mut"));
        assert!(authority_field.constraints.iter().any(|c| c.constraint_type == "signer"));
        
        // Check that mut was parsed correctly for from
        let from_field = account.find_field("from").unwrap();
        assert!(from_field.constraints.iter().any(|c| c.constraint_type == "mut"));
        
        // Check that to doesn't have mut
        let to_field = account.find_field("to").unwrap();
        assert!(!to_field.constraints.iter().any(|c| c.constraint_type == "mut"));
    }
}