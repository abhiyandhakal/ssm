use crate::utils::{
    fs::save_alias_list_to_file,
    parse::{insert_alias, parse_alias_config},
    tmux::get_tmux_start_dir,
};

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

    Ok(())
}
