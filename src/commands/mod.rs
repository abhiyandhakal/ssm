pub mod session;

use crate::cli::Cli;

pub fn execute(cli: Cli) -> std::io::Result<()> {
    if cli.save {
        // Save session
    } else if let Some(_new_alias) = cli.set_alias {
        // Set alias
    } else if cli.list_aliases {
        //
    } else if cli.find {
        if cli.is_directory {
            //
        } else if cli.is_both_alias_and_dir {
            //
        } else {
            // Find alias
        }
    } else if let Some(_path_or_alias) = cli.path_or_alias {
        //
    }

    Ok(())
}
