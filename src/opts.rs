use std::fmt::{Display, Formatter};
use std::path::Path;
use std::str::FromStr;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(name = "rcli-csv", author, version, about)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(about = "Convert CSV to JSON")]
    Csv(CsvOpts),
}

#[derive(Debug,Copy, Clone)]
pub enum OutputFormat{
    Json,
    Yaml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long,value_parser=file_validator)]
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

fn file_validator(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err(format!("File {} does not exist", filename))
    }
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
