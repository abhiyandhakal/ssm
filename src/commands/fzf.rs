use std::{io::Result, path::PathBuf};

use crate::utils::{
    find::{find_command, run_fzf},
    parse::parse_alias_config,
};

use super::session::open_session;

/// Use fuzzy finder to select alias
pub fn find_alias(restore_session: bool) -> Result<()> {
    let alias_list = parse_alias_config()?
        .iter()
        .map(|f| format!("{}\t\t{}", f.alias, f.path))
        .collect::<Vec<_>>()
        .join("\n");
    let alias_path_pair = run_fzf(format!("echo \"{alias_list}\""))?;
    let alias = alias_path_pair.split('\t').collect::<Vec<_>>()[0];
    open_session(alias.to_string(), restore_session)?;

    Ok(())
}

/// Use fuzzy finder to select path of directory
pub fn find_directory(show_hidden: bool, restore_session: bool) -> Result<()> {
    let find_command = find_command(show_hidden);
    let path = run_fzf(find_command)?;
    open_session(path, restore_session)?;

    Ok(())
}

/// Use fuzzy finder to select path of directory or alias (shows list of both)
pub fn find_both(show_hidden: bool, restore_session: bool) -> Result<()> {
    let alias_list = parse_alias_config()?
        .iter()
        .map(|f| format!("{}\t\t{}", f.alias, f.path))
        .collect::<Vec<_>>()
        .join("\n");
    let alias_or_path = run_fzf(format!(
        "({}; echo \"{}\";)",
        find_command(show_hidden),
        alias_list
    ))?;

    if PathBuf::from(&alias_or_path).is_dir() {
        open_session(alias_or_path, restore_session)?;
    } else {
        let alias = alias_or_path.split('\t').collect::<Vec<_>>()[0];
        open_session(alias.to_string(), restore_session)?;
    }

    Ok(())
}
