mod commands;
mod find;
mod tmux;
use std::io::Result;

use commands::{CommandEnum, CommandType};
use find::Find;

use crate::commands::Commands;

fn main() -> Result<()> {
    let commands_in_args = Commands::get_commands_in_args();

    // get directory
    let directory = if commands_in_args.contains(&(CommandEnum::Fzf, CommandType::Find)) {
        Find::get_directory(commands_in_args)
    } else {
        unimplemented!();
    }?;

    // go to tmux session
    tmux::goto_session(&directory);

    Ok(())
}
