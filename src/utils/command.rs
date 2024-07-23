use std::{
    io::{Error, Result},
    process::Command,
};

use super::tmux::is_in_tmux_session;

/// Execute the command passed as an argument and return the output
pub fn execute_command<T: AsRef<str>>(command: T) -> Result<String> {
    if !is_in_tmux_session() {
        return Err(Error::new(
            std::io::ErrorKind::NotFound,
            "Not in a tmux session",
        ));
    }
    let command = command.as_ref();
    let output = Command::new("sh").arg("-c").arg(command).output()?;
    let output = String::from_utf8_lossy(&output.stdout);
    let output = output.trim();

    Ok(output.to_string())
}
