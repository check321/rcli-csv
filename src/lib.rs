mod cli;
mod process;

pub use cli::{Opts,SubCommand,Base64Format,Base64SubCommand};
pub use process::{csv_process,process_genpass,process_encode,process_decode};