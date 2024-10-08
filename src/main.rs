use clap::Parser;
use rcli_csv::{csv_process, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output{
                output.clone()
            } else {
                format!("output.{}",opts.format)
            };
            csv_process(&opts.input,output,opts.format)?
        }
    }
    Ok(())
}
