use crate::commands::{CommandEnum, CommandType};
use std::io::Result;
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

    pub fn get_directory(flags: Vec<(CommandEnum, CommandType)>) {
        let find_flags: Vec<_> = flags
            .iter()
            .filter(|(_, t)| t == &CommandType::Find)
            .collect::<Vec<&(CommandEnum, CommandType)>>()
            .iter()
            .map(|f| f.0)
            .collect();

        let fd_exists = match Command::new("fd").spawn() {
            Ok(_) => true,
            Err(_) => false,
        };

        let command = if flags.contains(&(CommandEnum::Find, CommandType::Find)) || !fd_exists {
            Self::find_command(find_flags)
        } else {
            "".to_string()
        };

        println!("{}", command);
    }
}
