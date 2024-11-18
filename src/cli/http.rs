use super::parse_output;
use std::path::PathBuf;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(about = "Serve a dic via http protocol.")]
    Serve(HttpServOpts),

}

#[derive(Debug,Parser)]
pub struct HttpServOpts {
    #[arg(short,long,value_parser=parse_output,default_value = ".")]
    pub dir: PathBuf,

    // #[arg(long,default_value = "127.0.0.1")]
    // pub host: String,

    #[arg(long,default_value = "7070")]
    pub port: u16,
}