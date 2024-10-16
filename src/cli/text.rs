use std::fmt::{Display, Formatter};
use std::str::FromStr;
use clap::Parser;
use crate::Base64Format;
use super::{filename_parser};

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "Sign a message with a private key.")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signed message.")]
    Verify(TextVerifyOpts),

}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short,long,value_parser=filename_parser,default_value = "-")]
    pub input: String,

    #[arg(long)]
    pub key: String,

    #[arg(long,default_value = "blake3",value_parser=parse_format)]
    pub format: TextSignFormat,
}
#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short,long,value_parser=filename_parser,default_value = "-")]
    pub input: String,

    #[arg(long)]
    pub key: String,

    #[arg(long)]
    pub signature: String,

    #[arg(long,default_value = "blake3",value_parser=parse_format)]
    pub format: TextSignFormat,

}

#[derive(Debug,Copy,Clone)]
pub enum TextSignFormat{
    Blake3,
    Ed25519
}

fn parse_format(s: &str) -> Result<TextSignFormat, anyhow::Error> {
    s.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format"))
        }
    }
}

impl From<TextSignFormat> for &'static str{
    fn from(format: TextSignFormat) -> Self {
        match format{
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519"
        }
    }
}

impl Display for TextSignFormat{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",Into::<&str>::into(*self))
    }

}
