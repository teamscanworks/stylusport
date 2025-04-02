use std::path::PathBuf;
use std::str::FromStr;
use clap::ArgMatches;
use crate::error::Error;

/// Output format options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Yaml,
    Json,
    Debug,
}

impl FromStr for OutputFormat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "yaml" => Ok(OutputFormat::Yaml),
            "json" => Ok(OutputFormat::Json),
            "debug" => Ok(OutputFormat::Debug),
            _ => Err(Error::InvalidFormat(s.to_string())),
        }
    }
}

/// Configuration for command execution
#[derive(Debug)]
pub struct Config {
    pub input_path: PathBuf,
    pub output_path: Option<PathBuf>,
    pub format: OutputFormat,
}

impl Config {
    pub fn from_matches(matches: &ArgMatches) -> Result<Self, Error> {
        let input_path = matches.get_one::<String>("input")
            .ok_or_else(|| Error::MissingArgument("input".to_string()))?;

        let output_path = matches.get_one::<String>("output")
            .map(|s| PathBuf::from(s));

        let format = matches.get_one::<String>("format")
            .map(|s| OutputFormat::from_str(s))
            .transpose()?
            .unwrap_or(OutputFormat::Yaml);
    
            Ok(Config {
                input_path: PathBuf::from(input_path),
                output_path,
                format,
            })
    }
}