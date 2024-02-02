mod commands;
mod fzf;
use commands::{CommandEnum, CommandType};
use fzf::Fzf;

use crate::commands::Commands;

fn main() {
    let commands_in_args = Commands::get_commands_in_args();

    if commands_in_args.contains(&(CommandEnum::Fzf, CommandType::Fzf)) {
        Fzf::get_directory(CommandEnum::Fzf, commands_in_args);
    } else if commands_in_args.contains(&(CommandEnum::FzfShowHidden, CommandType::Fzf)) {
        Fzf::get_directory(CommandEnum::FzfShowHidden, commands_in_args);
    } else {
        println!("No command found");
    }
}
