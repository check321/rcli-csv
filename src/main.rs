use clap::Parser;
use rcli_csv::{csv_process, process_genpass, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            csv_process(&opts.input, output, opts.format)?;
        }
        SubCommand::Genpass(opts) => {
            process_genpass(opts.length as u8, opts.uppercase, opts.lowercase, opts.number, opts.symbols)?;
        }
    }
    Ok(())
}

