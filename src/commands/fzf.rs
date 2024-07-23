use std::io::Result;

use crate::utils::{fzf::run_fzf, parse::parse_alias_config};

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
