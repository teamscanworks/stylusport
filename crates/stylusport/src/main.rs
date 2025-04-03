use std::process;
use tracing::error;

mod cli;
mod commands;
mod config;
mod error;
mod output;

fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Build and parse CLI arguments
    let matches = cli::build_cli().get_matches();

    // Execute the selected command
    match cli::execute_command(&matches) {
        Ok(()) => {}
        Err(e) => {
            error!("{}", e);
            process::exit(1);
        }
    }
}
