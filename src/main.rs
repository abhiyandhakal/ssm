mod alias;
mod commands;
mod find;
mod session;
mod tmux;
mod utils;

use std::io::Result;

use commands::{CommandEnum, CommandType};
use find::Find;

use crate::commands::Commands;

fn main() -> Result<()> {
    let commands_in_args = Commands::get_commands_in_args();

    // check if save
    let is_save = commands_in_args
        .iter()
        .find(|(_, c)| *c == CommandType::Other);

    if is_save.is_some() {
        session::save_session()?;
    }

    // related to find
    let is_find = commands_in_args
        .iter()
        .find(|(_, c)| *c == CommandType::Find);

    // get directory
    if is_find.is_some() {
        let fzf_combo = match Commands::get_enum_type_combo(CommandEnum::Fzf) {
            Some(c) => c,
            None => {
                eprintln!("Command combination not found. Please report this issue");
                std::process::exit(1);
            }
        };
        let directory = if commands_in_args.contains(&fzf_combo) {
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
        match alias::set_alias() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
            }
        };
    }

    Ok(())
}
