mod csv;
mod genpass;
mod base64;
mod text;

use std::fmt::{Display, Formatter};
use std::path::Path;
use std::str::FromStr;
use clap::Parser;
use crate::cli::csv::CsvOpts;
use crate::cli::genpass::GenPassOpts;


pub use self::{csv::OutputFormat, base64::{Base64Format,Base64SubCommand},text::{TextSubCommand,TextSignFormat}};

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
    #[command(about = "Generate a random password")]
    Genpass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Text(TextSubCommand),
}

fn filename_parser(filename: &str) -> Result<String, String> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err(format!("File {} does not exist", filename))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_file_validator() {
        assert_eq!(filename_parser("assets/top250_movie.csv"), Ok("assets/top250_movie.csv".into()));
        assert_eq!(filename_parser("-"), Ok("-".into()));
        assert_eq!(filename_parser("nonexistent.csv"), Err("File nonexistent.csv does not exist".into()));
    }
}


