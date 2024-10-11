use std::fmt::{Display, Formatter};
use std::str::FromStr;
use clap::Parser;
use super::{filename_parser};

#[derive(Debug,Parser)]
pub enum Base64SubCommand{
    #[command(name = "encode",about="Encode string to Base64 format.")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode",about="Decode Base64 format to plain string.")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug,Parser)]
pub struct Base64EncodeOpts{
    #[arg(short,long,value_parser=filename_parser,default_value = "-")]
    pub input:String,

    #[arg(long,value_parser=b64_format_parser,default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug,Parser)]
pub struct Base64DecodeOpts{

    #[arg(short,long,value_parser=filename_parser,default_value = "-")]
    pub input:String,

    #[arg(short,long)]
    pub output:Option<String>,

    #[arg(long,value_parser=b64_format_parser,default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug,Clone,Copy)]
pub enum Base64Format{
    Standard,
    UrlSafe
}

fn b64_format_parser(format:&str)->Result<Base64Format, anyhow::Error>{
    format.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s{
            "standard"=>Ok(Base64Format::Standard),
            "urlsafe"=>Ok(Base64Format::UrlSafe),
            _=>Err(anyhow::anyhow!("Invalid format"))
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format{
            Base64Format::Standard=>"standard",
            Base64Format::UrlSafe=>"urlsafe"
        }
    }
}

impl Display for Base64Format{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",Into::<&str>::into(*self))
    }

}