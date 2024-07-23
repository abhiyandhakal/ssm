pub mod alias;
pub mod session;

use alias::{list_aliases, set_alias};
use session::open_session;

use crate::cli::Cli;

/// Execute the CLI
pub fn execute(cli: Cli) -> std::io::Result<()> {
    if cli.save {
        // Save session
    } else if let Some(new_alias) = cli.set_alias {
        // Set alias
        set_alias(new_alias)?
    } else if cli.list_aliases {
        // List aliases
        list_aliases()?
    } else if cli.find {
        if cli.is_directory {
            //
        } else if cli.is_both_alias_and_dir {
            //
        } else {
            // Find alias
        }
    } else if let Some(path_or_alias) = cli.path_or_alias {
        open_session(path_or_alias)?;
    }

    Ok(())
}
