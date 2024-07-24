use std::{
    io::{Read, Result},
    path::PathBuf,
    process::{Command, Stdio},
};

use dirs::home_dir;

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

/// Return the command required to find the directory
pub fn find_command(show_hidden: bool) -> String {
    let fd_exists = match Command::new("fd").output() {
        Ok(_) => true,
        Err(_) => false,
    };

    let home_dir = match home_dir() {
        Some(path) => path,
        None => PathBuf::from("."),
    };

    if fd_exists {
        let mut command = format!(
            "fd --type d --search-path {}",
            home_dir.to_str().unwrap_or(".")
        )
        .to_string();

        if show_hidden {
            command.push_str(" -H");
        }

        command
    } else {
        let mut command = format!("find {} -type d", home_dir.to_str().unwrap_or(".")).to_string();

        if show_hidden {
            command.push_str(" -name '.*'");
        } else {
            // Ignore hidden directories
            command.push_str(" -not -path '*/\\.*'");
        }

        command
    }
}
