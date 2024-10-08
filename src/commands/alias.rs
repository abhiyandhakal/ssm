use std::io::Result;

use crate::utils::{
    command::execute_command,
    fs::{get_alias_file, save_alias_list_to_file},
    parse::{insert_alias, parse_alias_config},
    tmux::get_current_session_start_dir,
};
use prettytable::{format, Table};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Alias {
    pub alias: String,
    pub path: String,
}

pub fn set_alias(new_alias: String) -> std::io::Result<()> {
    let tmux_start_dir = get_current_session_start_dir()?;

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
    if let Err(e) = execute_command(format!(
        "tmux rename-session -t \"{current_session}\" \"{new_alias}\""
    )) {
        eprintln!("{e}");
        std::process::exit(1);
    };

    Ok(())
}

/// Remove provided input from the alias list
pub fn remove_alias(alias: String) -> Result<()> {
    let mut alias_list = parse_alias_config()?;
    let mut alias_found = false;

    // Remove the saved alias
    for (i, alias_obj) in alias_list.clone().iter().enumerate() {
        if alias_obj.alias == alias {
            // Rename the tmux session to the absolute path
            execute_command(format!(
                "tmux rename-session -t {} {}",
                alias, alias_obj.path
            ))?;

            alias_list.remove(i);
            println!("Alias \"{alias}\" removed.");
            alias_found = true;
            break;
        }
    }

    if alias_found {
        save_alias_list_to_file(&alias_list)?;
    } else {
        // No alias removed
        eprintln!("Alias \"{alias}\" not found");
        std::process::exit(1);
    }

    Ok(())
}

/// Clear all the aliases
pub fn clear_aliases() -> Result<()> {
    let alias_file = get_alias_file()?;
    std::fs::write(alias_file, "[]")?;
    println!("All aliases cleared");

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
