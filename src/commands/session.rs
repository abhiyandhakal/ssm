use crate::utils::{
    command::execute_command,
    parse::parse_alias_config,
    tmux::{get_all_sessions, is_in_tmux_session},
};
use std::{
    io::{Error, Result},
    path::PathBuf,
};

use super::alias::Alias;

/// Open Session from path or alias
pub fn open_session(path_or_alias: String) -> Result<()> {
    let mut path_or_alias = path_or_alias;
    let mut session_name = path_or_alias.clone();
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
            // Convert path to absolute path
            match std::fs::canonicalize(&path_or_alias)?.to_str() {
                Some(path) => path_or_alias = path.to_owned(),
                None => {}
            };

            // Replace symbols that get transformed in tmux session name
            let replacable = [":", "."];

            session_name = path_or_alias
                .chars()
                .map(|c| {
                    if replacable.contains(&c.to_string().as_str()) {
                        "_".to_string()
                    } else if c == '\\' {
                        "\\\\".to_string()
                    } else {
                        c.to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join("");

            false
        }
    };
    let tmux_session_exists = all_sessions.iter().find(|f| **f == session_name).is_some();

    if !is_in_tmux_session {
        if tmux_session_exists {
            execute_command(match alias_found_saved {
                true => format!(
                    "tmux new -s {} -c {}",
                    &alias_path_pair.clone().unwrap().alias,
                    &alias_path_pair.unwrap().path
                ),
                false => format!("tmux new -s \"{session_name}\" -c \"{path_or_alias}\""),
            })?;
        } else {
            execute_command(format!("tmux attach -t \"{session_name}\""))?;
        }
    } else {
        if tmux_session_exists {
            execute_command(format!("tmux switch-client -t \"{session_name}\""))?;
        } else {
            execute_command(match alias_found_saved {
                false => format!("tmux new -s \"{session_name}\" -c \"{path_or_alias}\" -d && tmux switch-client -t \"{session_name}\""),
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
            eprintln!("{e}");
            std::process::exit(1);
        }
    };

    // Return the alias object if an alias matches
    for alias_obj in &alias_config_vec {
        if alias_obj.alias == input {
            return Some(alias_obj.clone());
        }

        if let Ok(path) = std::fs::canonicalize(input) {
            let path = path.to_string_lossy().to_string();
            if alias_obj.path == path {
                return Some(alias_obj.clone());
            }
        }
    }

    None
}
