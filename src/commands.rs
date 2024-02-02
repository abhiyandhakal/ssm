#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandEnum {
    Fzf,
    Help,
    FdShowHidden,
    Directory,
}

#[derive(Debug, Clone)]
pub struct CommandArgs {
    command: CommandEnum,
    args: Vec<String>,
}

pub struct Commands;

impl Commands {
    fn all_commands() -> Vec<CommandArgs> {
        vec![
            CommandArgs {
                command: CommandEnum::Fzf,
                args: vec!["--fzf".to_string(), "-z".to_string()],
            },
            CommandArgs {
                command: CommandEnum::Help,
                args: vec!["--help".to_string(), "-h".to_string()],
            },
            CommandArgs {
                command: CommandEnum::FdShowHidden,
                args: vec![
                    "--show-hidden".to_string(),
                    "-s".to_string(),
                    "--hidden".to_string(),
                ],
            },
            CommandArgs {
                command: CommandEnum::Directory,
                args: vec!["--directory".to_string(), "-d".to_string()],
            },
        ]
    }

    pub fn get_commands_in_args() -> Vec<CommandEnum> {
        let args = std::env::args().collect::<Vec<String>>();
        let all_commands = Commands::all_commands();

        let mut commands = vec![];

        for arg in &args {
            for command in &all_commands {
                if command.args.contains(arg) {
                    commands.push(command.command);
                }
            }
        }

        if commands.is_empty() {
            commands.push(CommandEnum::Fzf);
        }

        commands
    }
}
