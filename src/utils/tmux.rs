use std::{
    io::{Error, Result},
    path::PathBuf,
};

use super::command::execute_command;

/// Check if you are inside a tmux session
pub fn is_in_tmux_session() -> bool {
    if std::env::var("TMUX").is_err() {
        return false;
    }
    true
}

/// Get the start/active directory of the current Tmux session if exists
pub fn get_tmux_start_dir() -> std::io::Result<PathBuf> {
    let output = execute_command("tmux display-message -p -F '#{pane_current_path}'")?;

    if output.is_empty() {
        return Err(Error::new(
            std::io::ErrorKind::NotFound,
            "Tmux session unavailable",
        ));
    }

    // Standardize the path
    std::fs::canonicalize(&output)
}

/// Get a list of names of all the tmux sessions
pub fn get_all_sessions() -> Result<Vec<String>> {
    let output = execute_command("tmux ls")?;
    if output.is_empty() {
        return Ok(vec![]);
    }
    Ok(output
        .split('\n') // Split the new lines into vector
        .collect::<Vec<_>>()
        .iter()
        .map(|session| session.split(' ').collect::<Vec<_>>()[0]) // Get the names only (returns with a colon at the end)
        .collect::<Vec<_>>()
        .iter()
        .map(|f| &f[..f.len().saturating_sub(1)]) // Removes the colon at the end
        .collect::<Vec<_>>()
        .iter()
        .map(|f| f.to_string()) // Converts to String type
        .collect::<Vec<_>>())
}
