use clap::Parser;

use crate::cli::Cli;

mod cli;
mod git;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::Get(_) => todo!(),
    }
}
