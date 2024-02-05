mod alias;
mod commands;
mod find;
mod tmux;
use std::io::Result;

use commands::{CommandEnum, CommandType};
use find::Find;

use crate::commands::Commands;

fn main() -> Result<()> {
    let commands_in_args = Commands::get_commands_in_args();

    // related to find
    let is_find = commands_in_args
        .iter()
        .find(|(_, c)| *c == CommandType::Find);

    // get directory
    if is_find.is_some() {
        let directory = if commands_in_args.contains(&(CommandEnum::Fzf, CommandType::Find)) {
            Find::get_directory(commands_in_args)
        } else {
            unimplemented!();
        }?;

        // go to tmux session
        match tmux::goto_session(&directory) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
            }
        };

        return Ok(());
    }

    // related to alias
    let is_set_alias = commands_in_args
        .iter()
        .find(|(c, _)| *c == CommandEnum::SetAlias);

    if is_set_alias.is_some() {
        alias::set_alias();
    }

    Ok(())
}
