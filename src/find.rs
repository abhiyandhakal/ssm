use crate::commands::{CommandEnum, CommandType};
use std::io::{Read, Result};
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct Find;

impl Find {
    fn find_command(find_flags: Vec<CommandEnum>) -> String {
        let home_dir = match dirs::home_dir() {
            Some(dir) => dir,
            None => PathBuf::from("."),
        };

        let mut command = format!("find {} -type d", home_dir.to_str().unwrap_or(".")).to_string();

        if find_flags.contains(&CommandEnum::ShowHidden) {
            command.push_str(" -name '.*'");
        } else {
            // ignore hidden directories
            command.push_str(" -not -path '*/\\.*'");
        }

        command
    }

    fn fd_command(find_flags: Vec<CommandEnum>) -> String {
        let home_dir = match dirs::home_dir() {
            Some(dir) => dir,
            None => PathBuf::from("."),
        };

        let mut command = format!(
            "fd --type d --search-path {}",
            home_dir.to_str().unwrap_or(".")
        )
        .to_string();

        if find_flags.contains(&CommandEnum::ShowHidden) {
            command.push_str(" -H");
        }

        command
    }

    pub fn get_directory(flags: Vec<(CommandEnum, CommandType)>) -> Result<()> {
        let find_flags: Vec<_> = flags
            .iter()
            .filter(|(_, t)| t == &CommandType::Find)
            .collect::<Vec<&(CommandEnum, CommandType)>>()
            .iter()
            .map(|f| f.0)
            .collect();

        let fd_exists = match Command::new("fd").output() {
            Ok(_) => true,
            Err(_) => false,
        };

        let command = if flags.contains(&(CommandEnum::Find, CommandType::Find)) || !fd_exists {
            Self::find_command(find_flags)
        } else {
            Self::fd_command(find_flags)
        };

        let mut child = Command::new("sh")
            .arg("-c")
            .arg(format!("{} | fzf", command))
            .stdin(Stdio::inherit())
            .stdout(Stdio::piped())
            .spawn()?;

        let status = child.wait()?;
        if !status.success() {
            return Err(std::io::Error::from_raw_os_error(status.code().unwrap()));
        }

        let mut output = String::new();

        match child.stdout {
            Some(mut stdout) => {
                let _ = stdout
                    .read_to_string(&mut output)
                    .map_err(|e| e.to_string());
            }
            None => panic!("No output"),
        }

        Ok(())
    }
}
