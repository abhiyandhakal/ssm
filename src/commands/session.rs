use uuid::Uuid;

use crate::utils::{
    command::execute_command,
    fs::save_session_to_file,
    parse::parse_sessions_in_files,
    session::{get_windows, Session},
    tmux::{
        get_all_sessions, get_current_session, get_current_session_start_dir, is_in_tmux_session,
    },
    utils::browse_alias,
};
use std::{
    io::{Error, Result},
    path::PathBuf,
};

/// Open Session from path or alias
pub fn open_session(path_or_alias: String) -> Result<()> {
    let mut path_or_alias = path_or_alias;
    let session_name;
    let is_in_tmux_session = is_in_tmux_session();
    let all_sessions = get_all_sessions()?;
    let alias_path_pair = browse_alias(path_or_alias.as_str());
    let alias_found_saved = match &alias_path_pair {
        Some(pair) => {
            session_name = pair.alias.clone();
            true
        }
        None => {
            // Check if valid directory path if alias doesn't exist
            if !PathBuf::from(&path_or_alias).is_dir() {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Provided path doesn't exist.",
                ));
            }
            // Convert path to absolute path
            if let Some(path) = std::fs::canonicalize(&path_or_alias)?.to_str() {
                path_or_alias = path.to_owned()
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
    let tmux_session_exists = all_sessions.iter().any(|f| *f == session_name);

    if !is_in_tmux_session {
        if !tmux_session_exists {
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
    } else if tmux_session_exists {
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

    Ok(())
}

/// Save the tmux session state
pub fn save_session() -> Result<()> {
    let current_session = get_current_session()?;
    let windows = get_windows(&current_session)?;
    let session: Session = Session {
        windows,
        start_dir: get_current_session_start_dir()?
            .to_string_lossy()
            .to_string(),
        name: get_current_session()?,
    };
    let sessions_saved_with_filename = parse_sessions_in_files()?;

    let mut saved_before_file = None;

    for (filename, session) in sessions_saved_with_filename {
        if session.name == current_session {
            saved_before_file = Some(filename);
            break;
        }
    }

    let filename = if let Some(saved_before_path) = saved_before_file {
        saved_before_path
    } else {
        Uuid::new_v4().to_string()
    };

    save_session_to_file(session, filename)?;

    Ok(())
}
