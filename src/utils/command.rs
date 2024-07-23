use std::{io::Result, process::Command};

/// Execute the command passed as an argument and return the output
pub fn execute_command<T: AsRef<str>>(command: T) -> Result<String> {
    let command = command.as_ref();
    let output = Command::new("sh").arg("-c").arg(command).output()?;
    let output = String::from_utf8_lossy(&output.stdout);
    let output = output.trim();

    Ok(output.to_string())
}
