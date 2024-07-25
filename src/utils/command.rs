use std::{
    io::{ErrorKind, Result},
    process::{Command, Stdio},
};

/// Execute the command passed as an argument and return the output
pub fn execute_command<T: AsRef<str>>(command: T) -> Result<String> {
    let command = command.as_ref();
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdin(Stdio::inherit())
        .output()?;

    // Check if error
    if output.stderr.len() != 0 {
        return Err(std::io::Error::new(
            ErrorKind::UnexpectedEof,
            String::from_utf8_lossy(&output.stderr),
        ));
    }

    let output = String::from_utf8_lossy(&output.stdout);
    let output = output.trim();

    Ok(output.to_string())
}
