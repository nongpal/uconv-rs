mod converter;

use clap::{Parser, Subcommand};

enum Suhu {
    Celcius,
    Fahrenheit,
    Kelvin,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    ///
    From { desc: String },

    /// Display all available unit
    List,

    /// A value to convert to.
    /// Can be integer or float
    Value { id: f64 },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[command(subcommand)]
    command: Cmd,
}

fn main() {
    let args = Args::parse();
}
