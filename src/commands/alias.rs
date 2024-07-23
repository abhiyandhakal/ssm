use crate::utils::{
    command::execute_command,
    fs::save_alias_list_to_file,
    parse::{insert_alias, parse_alias_config},
    tmux::get_tmux_start_dir,
};
use prettytable::{format, Table};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Alias {
    pub alias: String,
    pub path: String,
}

pub fn set_alias(new_alias: String) -> std::io::Result<()> {
    let tmux_start_dir = get_tmux_start_dir()?;

    let alias = Alias {
        alias: new_alias.to_string(),
        path: tmux_start_dir.to_string_lossy().to_string(),
    };

    let mut alias_config_vec = parse_alias_config()?;
    insert_alias(alias, &mut alias_config_vec);
    save_alias_list_to_file(&alias_config_vec)?;

    println!("{}: {}", new_alias, tmux_start_dir.to_string_lossy());
    println!("Alias saved");

    // Also update the current session name
    let current_session = execute_command("tmux display-message -p -F '#{session_name}'")?;
    match execute_command(format!(
        "tmux rename-session -t \"{current_session}\" \"{new_alias}\""
    )) {
        Err(e) => {
            eprintln!("{}", e.to_string());
            std::process::exit(1);
        }
        _ => {}
    };

    Ok(())
}

/// Print all the aliases saved
pub fn list_aliases() -> std::io::Result<()> {
    let alias_config_vec = parse_alias_config()?;

    // Create a table
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    // Add the headers to the table
    table.set_titles(row![
        bFy->"Alias", bFy->"Path"
    ]);

    // Add aliases to the table
    for alias_obj in alias_config_vec {
        table.add_row(row![
            Fg->alias_obj.alias, Fc->alias_obj.path
        ]);
    }

    // Display the table
    table.printstd();

    Ok(())
}
