use std::{io::Error, path::PathBuf, process::Command};

/// Get the start/active directory of the current Tmux session if exists
pub fn get_tmux_start_dir() -> std::io::Result<PathBuf> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("tmux display-message -p -F '#{pane_current_path}'")
        .output()?;

    let output = String::from_utf8_lossy(&output.stdout);
    let output = output.trim();

    if output.is_empty() {
        return Err(Error::new(
            std::io::ErrorKind::NotFound,
            "Tmux session unavailable",
        ));
    }

    Ok(PathBuf::from(&output))
}
