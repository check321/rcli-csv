use clap::Parser;
use rcli_csv::{csv_process, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            csv_process(&opts.input, &opts.output)?
        }
    }
    Ok(())
}
