use std::path::Path;
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

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long,value_parser=file_validator)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,

}

fn file_validator(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err(format!("File {} does not exist", filename))
    }
}