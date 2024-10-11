use std::fmt::{Display, Formatter};
use std::str::FromStr;
use clap::Parser;
use super::filename_parser;

#[derive(Debug,Copy, Clone)]
pub enum OutputFormat{
    Json,
    Yaml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long,value_parser=filename_parser)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,

    #[arg(long, default_value_t = OutputFormat::Json,value_parser=output_format_validator)]
    pub format: OutputFormat,

}

fn output_format_validator(format: &str) -> Result<OutputFormat, anyhow::Error> { format.parse()}

impl From<OutputFormat> for &'static str{
    fn from(format: OutputFormat) -> &'static str {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}
impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid output format")),
        }
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",Into::<&str>::into(*self))
    }
}