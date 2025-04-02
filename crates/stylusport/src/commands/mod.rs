use clap::{ArgMatches, Command as ClapCommand};
use crate::error::Error;

pub mod parse;
// Future command modules
// pub mod normalize;
// pub mod build_ir;

pub trait Command {
    /// Returns the name of the command (used in CLI)
    fn name(&self) -> &'static str;
    
    /// Builds the clap subcommand for this command
    fn build_subcommand(&self) -> ClapCommand;
    
    /// Executes the command with the given arguments
    fn execute(&self, matches: &ArgMatches) -> Result<(), Error>;
}

/// Returns all available commands
pub fn get_all_commands() -> Vec<Box<dyn Command>> {
    vec![
        Box::new(parse::ParseCommand),
        // Add more commands as they're implemented
        // Box::new(normalize::NormalizeCommand),
    ]
}