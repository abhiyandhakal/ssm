pub mod alias;
pub mod fzf;
pub mod session;

use alias::{clear_aliases, list_aliases, remove_alias, set_alias};
use fzf::{find_alias, find_both, find_directory};
use session::open_session;

use crate::cli::Cli;

/// Execute the CLI
pub fn execute(cli: Cli) -> std::io::Result<()> {
    if cli.save {
        // Save session
    } else if let Some(new_alias) = cli.set_alias {
        // Set alias
        set_alias(new_alias)?
    } else if let Some(old_alias) = cli.remove_alias {
        remove_alias(old_alias)?
    } else if cli.list_aliases {
        // List aliases
        list_aliases()?
    } else if cli.clear_alias {
        clear_aliases()?
    } else if cli.find {
        if cli.is_directory {
            find_directory(cli.show_hidden)?
        } else if cli.is_both_alias_and_dir {
            find_both(cli.show_hidden)?
        } else {
            // Find alias
            find_alias()?
        }
    } else if let Some(path_or_alias) = cli.path_or_alias {
        open_session(path_or_alias)?;
    }

    Ok(())
}
