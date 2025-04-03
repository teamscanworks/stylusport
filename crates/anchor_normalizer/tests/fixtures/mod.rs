//! Test fixtures for normalization tests

use anchor_parser::model::{
    Account, AccountField, Constraint, Instruction, Parameter, Program, ProgramModule, RawAccount,
    RawAccountField,
};

/// Create a simple hello world program fixture
pub fn hello_world_program() -> Program {
    let mut program = Program::new();

    // Create the program module
    let mut module = ProgramModule::new("hello_world", "");

    // Create the initialize instruction
    let mut instruction = Instruction::new("initialize", "pub");
    instruction.add_parameter(Parameter::new_context("ctx", "Initialize"));
    instruction.set_return_type("Result<()>");
    instruction.set_context_type("Initialize");

    // Add instruction to module
    module.add_instruction(instruction);

    // Add module to program
    program.add_program_module(module);

    // Create account struct
    let account = Account::new("Initialize", "pub");

    // Add account to program
    program.add_account_struct(account);

    program
}

/// Create a token program fixture
pub fn token_program() -> Program {
    let mut program = Program::new();

    // Create the program module
    let mut module = ProgramModule::new("token_program", "pub");

    // Create the initialize instruction
    let mut init_instruction = Instruction::new("initialize", "pub");
    init_instruction.add_parameter(Parameter::new_context("ctx", "Initialize"));
    init_instruction.set_return_type("Result<()>");
    init_instruction.set_context_type("Initialize");

    // Create the mint instruction
    let mut mint_instruction = Instruction::new("mint", "pub");
    mint_instruction.add_parameter(Parameter::new_context("ctx", "Mint"));
    mint_instruction.add_parameter(Parameter::new("amount", "u64", false));
    mint_instruction.set_return_type("Result<()>");
    mint_instruction.set_context_type("Mint");

    // Create the transfer instruction
    let mut transfer_instruction = Instruction::new("transfer", "pub");
    transfer_instruction.add_parameter(Parameter::new_context("ctx", "Transfer"));
    transfer_instruction.add_parameter(Parameter::new("amount", "u64", false));
    transfer_instruction.set_return_type("Result<()>");
    transfer_instruction.set_context_type("Transfer");

    // Add instructions to module
    module.add_instruction(init_instruction);
    module.add_instruction(mint_instruction);
    module.add_instruction(transfer_instruction);

    // Add module to program
    program.add_program_module(module);

    // Create Initialize account struct
    let mut init_account = Account::new("Initialize", "pub");

    // Add an authority field with signer constraint
    let mut authority_field = AccountField::new("authority", "Signer<'info>");
    authority_field.add_constraint(Constraint::without_value("signer"));
    init_account.add_field(authority_field);

    // Add a mint field with init constraint
    let mut mint_field = AccountField::new("mint", "Account<'info, Mint>");
    mint_field.add_constraint(Constraint::without_value("init"));
    mint_field.add_constraint(Constraint::with_value("payer", "authority"));
    init_account.add_field(mint_field);

    // Add a system program field
    init_account.add_field(AccountField::new(
        "system_program",
        "Program<'info, System>",
    ));

    // Create Mint account struct
    let mut mint_account = Account::new("Mint", "pub");

    // Add fields to Mint account
    let mut authority_field = AccountField::new("authority", "Signer<'info>");
    authority_field.add_constraint(Constraint::without_value("signer"));
    mint_account.add_field(authority_field);

    let mut mint_field = AccountField::new("mint", "Account<'info, Mint>");
    mint_field.add_constraint(Constraint::without_value("mut"));
    mint_account.add_field(mint_field);

    let mut to_field = AccountField::new("to", "Account<'info, TokenAccount>");
    to_field.add_constraint(Constraint::without_value("mut"));
    mint_account.add_field(to_field);

    // Create Transfer account struct
    let mut transfer_account = Account::new("Transfer", "pub");

    // Add fields to Transfer account
    let mut authority_field = AccountField::new("authority", "Signer<'info>");
    authority_field.add_constraint(Constraint::without_value("signer"));
    transfer_account.add_field(authority_field);

    let mut from_field = AccountField::new("from", "Account<'info, TokenAccount>");
    from_field.add_constraint(Constraint::without_value("mut"));
    transfer_account.add_field(from_field);

    let mut to_field = AccountField::new("to", "Account<'info, TokenAccount>");
    to_field.add_constraint(Constraint::without_value("mut"));
    transfer_account.add_field(to_field);

    // Add account structs to program
    program.add_account_struct(init_account);
    program.add_account_struct(mint_account);
    program.add_account_struct(transfer_account);

    // Create TokenAccount raw account
    let mut token_account = RawAccount::new("TokenAccount", "pub");
    token_account.add_field(RawAccountField::new("owner", "Pubkey", "pub"));
    token_account.add_field(RawAccountField::new("amount", "u64", "pub"));

    // Create Mint raw account
    let mut mint_raw = RawAccount::new("Mint", "pub");
    mint_raw.add_field(RawAccountField::new("authority", "Pubkey", "pub"));
    mint_raw.add_field(RawAccountField::new("supply", "u64", "pub"));

    // Add raw accounts to program
    program.add_raw_account(token_account);
    program.add_raw_account(mint_raw);

    program
}

/// Creates a program with various validation issues for testing error handling
///
/// # Arguments
///
/// * `include_module` - Whether to include a program module
/// * `valid_instructions` - Whether instructions should be valid
///
/// # Returns
///
/// A Program with deliberately introduced issues for testing error handling
pub fn create_invalid_program(include_module: bool, valid_instructions: bool) -> Program {
    let mut program = Program::new();

    if include_module {
        let mut module = ProgramModule::new("invalid_program", "pub");

        if valid_instructions {
            // Add a valid instruction
            let instruction = Instruction::new("initialize", "pub")
                .with_parameter(Parameter::new_context("ctx", "Initialize"))
                .with_return_type("Result<()>");

            module.add_instruction(instruction);
        } else {
            // Add an invalid instruction (missing context)
            let instruction = Instruction::new("invalid", "pub")
                .with_parameter(Parameter::new("value", "u64", false))
                .with_return_type("Result<()>");

            module.add_instruction(instruction);
        }

        program.add_program_module(module);
    }

    // Add an account struct for reference
    let account = Account::new("Initialize", "pub");
    program.add_account_struct(account);

    program
}
