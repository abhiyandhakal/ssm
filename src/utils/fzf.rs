use std::{
    io::{Read, Result},
    process::{Command, Stdio},
};

/// Run fuzzy finder, provided the command, and return the result
pub fn run_fzf<T: AsRef<str>>(command: T) -> Result<String> {
    let command = command.as_ref();
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(format!("{command} | fzf"))
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .spawn()?;

    let status = child.wait()?;
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }

    let mut output = String::new();

    match child.stdout {
        Some(mut stdout) => {
            let _ = stdout
                .read_to_string(&mut output)
                .map_err(|e| e.to_string());
        }
        None => {
            eprintln!("No output from fzf");
            std::process::exit(1);
        }
    }

    let output = output.trim();

    Ok(output.to_string())
}
