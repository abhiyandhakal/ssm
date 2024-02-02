use crate::commands::{CommandEnum, CommandType};
use std::io::Result;
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct Fzf;

impl Fzf {
    fn find_command(operation_type: CommandEnum) -> String {
        let home_dir = match dirs::home_dir() {
            Some(dir) => dir,
            None => todo!(),
        };

        let mut command = format!("find {} -type d", home_dir.to_str().unwrap_or(".")).to_string();

        if operation_type == CommandEnum::FzfShowHidden {
            command.push_str(" -name '.*'");
        } else {
            // ignore hidden directories
            command.push_str(" -not -path '*/\\.*'");
        }

        command
    }

    pub fn get_directory(operation_type: CommandEnum, flags: Vec<(CommandEnum, CommandType)>) {
        let command = if flags.contains(&(CommandEnum::Find, CommandType::Fzf)) {
            Self::find_command(operation_type)
        } else {
            "".to_string()
        };

        println!("{}", command);
    }
}
