use std::fs;
use clap::Parser;
use rcli_csv::{csv_process, process_genpass, process_encode, process_decode, Opts, SubCommand, Base64SubCommand, TextSubCommand, process_sign, process_verify, process_generate, TextSignFormat, HttpSubCommand, process_http_serve};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
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
           let password = process_genpass(opts.length as u8, opts.uppercase, opts.lowercase, opts.number, opts.symbols)?;
            print!("{}",password);
        }
        SubCommand::Base64(sub_cmd) => match sub_cmd{
            Base64SubCommand::Encode(opts)=>{
               let encoded = process_encode(&opts.input,opts.format)?;
                print!("{}",encoded);
            }

            Base64SubCommand::Decode(opts)=> {
                let decoded = process_decode(&opts.input,opts.format)?;
                let decoded = String::from_utf8(decoded)?;
                print!("{}",decoded);
            }
        },
        SubCommand::Text(sub_cmd) => match sub_cmd {
            TextSubCommand::Sign(opts) => {
                let signed = process_sign(&opts.input, &opts.key,opts.format)?;
                print!("{}", signed);
            }
            TextSubCommand::Verify(opts) => {
                let verified = process_verify(&opts.input, &opts.key,&opts.signature,opts.format)?;
                print!("{}", verified);
            }
            TextSubCommand::GenerateKey(opts) => {
                let key = process_generate(opts.format)?;
                match opts.format{
                    TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3.key");
                        fs::write(name,&key[0])?;
                    }
                    TextSignFormat::Ed25519 => {
                        let name = &opts.output;
                        fs::write(name.join("ed25519.sk"),&key[0])?;
                        fs::write(name.join("ed25519.pk"),&key[1])?;
                    }
                }
            }
        },
        SubCommand::Http(sub_cmd) => match sub_cmd {
            HttpSubCommand::Serve(opts) => {
                process_http_serve(opts.dir,opts.port).await?;
            }
        }
    }
    Ok(())
}

