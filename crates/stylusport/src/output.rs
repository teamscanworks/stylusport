use crate::config::OutputFormat;
use crate::error::Error;
use serde::Serialize;
use std::io::Write;

/// Trait for types that can be displayed in different formats
pub trait Displayable: Serialize + std::fmt::Debug {
    fn to_string(&self, format: &OutputFormat) -> Result<String, Error> {
        match format {
            OutputFormat::Yaml => Ok(serde_yaml::to_string(self)?),
            OutputFormat::Json => Ok(serde_json::to_string_pretty(self)?),
            OutputFormat::Debug => Ok(format!("{:#?}", self)),
        }
    }

    fn write_to<W: Write>(&self, writer: &mut W, format: &OutputFormat) -> Result<(), Error> {
        let output = self.to_string(format)?;
        writer.write_all(output.as_bytes()).map_err(Error::IO)
    }
}

// Implementation for Program types from anchor_parser
impl Displayable for anchor_parser::Program {}
