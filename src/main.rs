use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rcli-csv", author, version, about)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(about = "Convert CSV to JSON")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long)]
    input: String,

    #[arg(short, long, default_value = "output.json")]
    output: String,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(long, default_value_t = true)]
    header: bool,

}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}",opts)
}
