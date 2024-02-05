pub fn get_alias() {}

fn get_alias_from_args() -> String {
    let args = std::env::args().collect::<Vec<String>>();
    let command_args = crate::Commands::get_command_by_enum(crate::commands::CommandEnum::SetAlias);

    if command_args.is_none() {
        eprintln!("No command found");
        std::process::exit(1);
    }
    let command_args = command_args.unwrap();

    let index = args
        .iter()
        .position(|arg| command_args.args.contains(arg))
        .unwrap_or_else(|| {
            eprintln!("No args found");
            std::process::exit(1);
        });

    if index + 1 >= args.len() {
        eprintln!("No alias found");
        std::process::exit(1);
    }

    let alias: String = args[index + 1].to_string();

    alias
}

pub fn set_alias() {
    let new_alias = get_alias_from_args();
}
