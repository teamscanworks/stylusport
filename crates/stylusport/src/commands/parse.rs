use super::Command;
use crate::config::Config;
use crate::error::Error;
use crate::output::Displayable;
use anchor_parser;
use clap::{Arg, ArgAction, ArgMatches, Command as ClapCommand};
use std::fs::File;
use std::io;

pub struct ParseCommand;

impl Command for ParseCommand {
    fn name(&self) -> &'static str {
        "parse"
    }

    fn build_subcommand(&self) -> ClapCommand {
        ClapCommand::new(self.name())
            .about("Parse Anchor code and output AST")
            .arg(Arg::new("input").help("Input file to parse").required(true))
            .arg(
                Arg::new("format")
                    .long("format")
                    .short('f')
                    .value_parser(["yaml", "json", "debug"])
                    .default_value("yaml")
                    .help("Output format"),
            )
            .arg(
                Arg::new("output")
                    .long("output")
                    .short('o')
                    .help("Output file (stdout if not specified)"),
            )
            .arg(
                Arg::new("verbose")
                    .short('v')
                    .long("verbose")
                    .action(ArgAction::Count)
                    .value_parser(clap::value_parser!(u8))
                    .help("Increase verbosity"),
            )
            .arg(
                Arg::new("quiet")
                    .short('q')
                    .long("quiet")
                    .help("Suppress all non-essential output")
                    .action(ArgAction::SetTrue)
                    .conflicts_with("verbose"),
            )
    }

    fn execute(&self, matches: &ArgMatches) -> Result<(), Error> {
        let config = Config::from_matches(matches)?;

        // Parse the input file
        tracing::info!("Parsing file: {:?}", config.input_path);
        let program = anchor_parser::parse_file(&config.input_path).map_err(Error::Parse)?;

        // Output the AST model based on the configured format and destination
        if let Some(output_path) = &config.output_path {
            // Write to file
            let mut file = File::create(output_path)?;
            program.write_to(&mut file, &config.format)?;
            tracing::info!("Output written to {:?}", output_path);
        } else {
            // Write to stdout
            program.write_to(&mut io::stdout(), &config.format)?;
        }

        Ok(())
    }
}
