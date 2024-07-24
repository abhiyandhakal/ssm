use std::io::Result;

use crate::utils::{
    find::{find_command, run_fzf},
    parse::parse_alias_config,
};

use super::session::open_session;

/// Use fuzzy finder to select alias
pub fn find_alias() -> Result<()> {
    let alias_list = parse_alias_config()?
        .iter()
        .map(|f| format!("{}\t\t{}", f.alias, f.path))
        .collect::<Vec<_>>()
        .join("\n");
    let alias_path_pair = run_fzf(format!("echo \"{alias_list}\""))?;
    let alias = alias_path_pair.split('\t').collect::<Vec<_>>()[0];
    open_session(alias.to_string())?;

    Ok(())
}

/// Use fuzzy finder to select path of directory
pub fn find_directory(show_hidden: bool) -> Result<()> {
    let find_command = find_command(show_hidden);
    let path = run_fzf(format!("{find_command}"))?;
    open_session(path)?;

    Ok(())
}
