use clap::Parser;
use rcli_csv::{csv_process, process_genpass, process_encode, process_decode, Opts, SubCommand, Base64SubCommand, TextSubCommand, TextSignFormat, process_sign,process_verify};

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
        SubCommand::Base64(subCmd) => match subCmd{
            Base64SubCommand::Encode(opts)=>{
                process_encode(&opts.input,opts.format)?;
            }

            Base64SubCommand::Decode(opts)=> {
                process_decode(&opts.input,opts.format)?;
            }
        },
        SubCommand::Text(subCmd) => match subCmd {
            TextSubCommand::Sign(opts) => {
                process_sign(&opts.input, &opts.key,opts.format)?
            }
            TextSubCommand::Verify(opts) => {
                process_verify(&opts.input, &opts.key,&opts.signature,opts.format)?
            }
        }
    }
    Ok(())
}

