use clap::Parser;

/// Smart Session Manager for Tmux
#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = None,
    override_usage = "ssm [PATH_OR_ALIAS] or ssm [OPTIONS]"
)]
pub struct Cli {
    /// Path of the directory or the alias you have saved as
    #[arg(index = 1)]
    pub path_or_alias: Option<String>,

    /// List all the aliases
    #[arg(short, long("list-alias"), action)]
    pub list_aliases: bool,

    /// Set alias to the active directory
    #[arg(short('a'), long("set-alias"))]
    pub set_alias: Option<String>,

    /// Save the current session
    #[arg(short, long, action)]
    pub save: bool,

    /// Start a new session, don't restore
    #[arg(short, long("new-session"), action)]
    pub new_session: bool,

    /// Find (default is alias)
    #[arg(short('f'), long("find"), action)]
    pub find: bool,

    /// Opt to path. Usage: ssm -f -d or ssm --find --dir
    #[arg(short('d'), long("dir"), action)]
    pub is_directory: bool,

    /// Show hidden directories. (use only with --dir). Usage: ssm -f -d -H or ssm --find --dir --show-hidden
    #[arg(short('H'), long("show-hidden"), action)]
    pub show_hidden: bool,

    /// Opt to both aliases and directories. Usage: ssm -f -A or ssm --find --all
    #[arg(short('A'), long("all"), action)]
    pub is_both_alias_and_dir: bool,
}
