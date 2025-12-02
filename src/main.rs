mod converter;
//mod history;

use clap::{Parser, Subcommand};
use converter::{run_conversion, run_list};

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Conversion command
    Convert {
        #[arg(long)]
        from: String,

        #[arg(long)]
        to: String,

        #[arg(long)]
        value: f64,
    },
    /// Display all available unit
    List,

    /// DIsplay history
    History,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[command(subcommand)]
    command: Cmd,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Cmd::Convert { from, to, value } => run_conversion(&from, &to, value),
        Cmd::List => run_list(),
        Cmd::History => todo!(),
    }
}
