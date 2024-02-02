mod commands;
mod find;
use commands::{CommandEnum, CommandType};
use find::Find;

use crate::commands::Commands;

fn main() {
    let commands_in_args = Commands::get_commands_in_args();

    if commands_in_args.contains(&(CommandEnum::Fzf, CommandType::Find)) {
        Find::get_directory(commands_in_args);
    }
}
