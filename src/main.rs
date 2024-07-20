mod args;

use clap::Parser;

fn main() {
    let args = args::Cli::parse();
}
