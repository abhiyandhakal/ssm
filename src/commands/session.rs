use std::{
    io::{Error, Result},
    path::PathBuf,
};

use crate::utils::{
    command::execute_command,
    parse::parse_alias_config,
    tmux::{get_all_sessions, is_in_tmux_session},
};

use super::alias::Alias;

/// Open Session from path or alias
/// TODO: Update the session name if there are symbols like '.' in the path
pub fn open_session(path_or_alias: String) -> Result<()> {
    let is_in_tmux_session = is_in_tmux_session();
    let all_sessions = get_all_sessions()?;
    let alias_path_pair = browse_alias(path_or_alias.as_str());
    let alias_found_saved = match alias_path_pair {
        Some(_) => true,
        None => {
            // Check if valid directory path if alias doesn't exist
            if !PathBuf::from(&path_or_alias).is_dir() {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Provided path doesn't exist.",
                ));
            }
            false
        }
    };
    let tmux_session_exists = match all_sessions.iter().find(|f| **f == path_or_alias) {
        Some(_) => true,
        None => false,
    };

    if !is_in_tmux_session {
        if tmux_session_exists {
            execute_command(match alias_found_saved {
                true => format!(
                    "tmux new -s {} -c {}",
                    &alias_path_pair.clone().unwrap().alias,
                    &alias_path_pair.unwrap().path
                ),
                false => format!("tmux new -s \"{path_or_alias}\" -c \"{path_or_alias}\""),
            })?;
        } else {
            execute_command(format!("tmux attach -t \"{path_or_alias}\""))?;
        }
    } else {
        if tmux_session_exists {
            execute_command(format!("tmux switch-client -t \"{path_or_alias}\""))?;
        } else {
            execute_command(match alias_found_saved {
                false => format!("tmux new -s \"{path_or_alias}\" -c \"{path_or_alias}\" -d && tmux switch-client -t \"{path_or_alias}\""),
                true => {
                    let alias_path_pair = alias_path_pair.unwrap();
                    format!(
                        "tmux new -s \"{}\" -c \"{}\" -d && tmux switch-client -t \"{}\"",
                        &alias_path_pair.alias,
                        &alias_path_pair.path,
                        &alias_path_pair.alias
                    )
                }
            })?;
        }
    }

    Ok(())
}

/// Search if the input exists in alias list. If exists, return the alias-path pair
fn browse_alias<T: AsRef<str>>(input: T) -> Option<Alias> {
    let input = input.as_ref();
    let alias_config_vec = match parse_alias_config() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", e.to_string());
            std::process::exit(1);
        }
    };

    // Return the alias object if an alias matches
    for alias_obj in &alias_config_vec {
        if alias_obj.alias == input {
            return Some(alias_obj.clone());
        }
    }

    None
}
