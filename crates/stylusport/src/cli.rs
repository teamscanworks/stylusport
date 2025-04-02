use clap::{Command as ClapCommand, ArgMatches};
use crate::commands;
use crate::error::Error;

/// Build the CLI parser
pub fn build_cli() -> ClapCommand {
    let mut app = ClapCommand::new("stylusport")
        .about("Solana Anchor to Stylus translator")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true);
    
    // Register all command subcommands
    for cmd in commands::get_all_commands() {
        app = app.subcommand(cmd.build_subcommand());
    }
    
    app
}

/// Execute the selected command
pub fn execute_command(matches: &ArgMatches) -> Result<(), Error> {
    // Get the subcommand name and matches
    let (subcmd_name, subcmd_matches) = matches.subcommand()
        .ok_or_else(|| Error::UnknownCommand("No subcommand provided".to_string()))?;
    
    // Find and execute the matching command
    for cmd in commands::get_all_commands() {
        if cmd.name() == subcmd_name {
            return cmd.execute(subcmd_matches);
        }
    }
    
    Err(Error::UnknownCommand(format!("Unknown command: {}", subcmd_name)))
}