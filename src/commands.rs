#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandType {
    Find,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandEnum {
    Fzf,
    Help,
    ShowHidden,
    Directory,
    Find,
}

#[derive(Debug, Clone)]
pub struct CommandArgs {
    command: CommandEnum,
    args: Vec<String>,
    command_type: CommandType,
}

pub struct Commands;

impl Commands {
    fn all_commands() -> Vec<CommandArgs> {
        vec![
            CommandArgs {
                command: CommandEnum::Fzf,
                args: vec!["--fzf".to_string(), "-z".to_string()],
                command_type: CommandType::Find,
            },
            CommandArgs {
                command: CommandEnum::Help,
                args: vec!["--help".to_string(), "-h".to_string()],
                command_type: CommandType::Other,
            },
            CommandArgs {
                command: CommandEnum::ShowHidden,
                args: vec![
                    "--show-hidden".to_string(),
                    "-s".to_string(),
                    "--hidden".to_string(),
                ],
                command_type: CommandType::Find,
            },
            CommandArgs {
                command: CommandEnum::Directory,
                args: vec!["--directory".to_string(), "-d".to_string()],
                command_type: CommandType::Other,
            },
            CommandArgs {
                command: CommandEnum::Find,
                args: vec!["--find".to_string(), "-f".to_string()],
                command_type: CommandType::Find,
            },
        ]
    }

    pub fn get_commands_in_args() -> Vec<(CommandEnum, CommandType)> {
        let args = std::env::args().collect::<Vec<String>>();
        let all_commands = Commands::all_commands();

        let mut commands = vec![];

        for arg in &args {
            for command in &all_commands {
                if command.args.contains(arg) {
                    commands.push((command.command, command.command_type));
                }
            }
        }

        if commands.is_empty() {
            commands.push((CommandEnum::Fzf, CommandType::Find));
        }

        commands
    }
}
