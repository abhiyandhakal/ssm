#[macro_use]
extern crate prettytable;
mod cli;
mod commands;
mod utils;

use clap::Parser;
use commands::execute;

fn main() -> std::io::Result<()> {
    let cli = cli::Cli::parse();

    match execute(cli) {
        Ok(_) => {}
        Err(e) => {
            // Print error and exit with status code 1
            eprint!("{e}");
            std::process::exit(1)
        }
    };

    Ok(())
}
