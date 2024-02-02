mod commands;
mod fzf;
use commands::CommandEnum;
use fzf::Fzf;

use crate::commands::Commands;

fn main() {
    let commands_in_args = Commands::get_commands_in_args();

    if commands_in_args.contains(&CommandEnum::Fzf) {
        Fzf::run().unwrap();
    } else if commands_in_args.contains(&CommandEnum::FdShowHidden) {
        Fzf::run_hidden();
    } else {
        println!("No command found");
    }
}
