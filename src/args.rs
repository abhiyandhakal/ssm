use clap::Parser;

/// Smart Session Manager for Tmux
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
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

    /// Show hidden directories
    #[arg(short('H'), long("show-hidden"), action)]
    pub show_hidden: bool,

    /// Start a new session, don't restore
    #[arg(short, long("new-session"), action)]
    pub new_session: bool,

    /// Find (default is alias), aliases = ["find", "find-aliases", "f", "fa"]
    #[arg(short('F'), long("find-alias"), aliases = &["find", "find-aliases", "f", "fa"], action)]
    pub find_aliases: bool,

    /// Find directories, aliases = ["find-dirs", "fd"]
    #[arg(short('D'), long("find-dir"), aliases = &["find-dirs", "fd"], action)]
    pub find_directories: bool,

    /// Find both aliases and directories
    #[arg(short('A'), long("find-all"), action)]
    pub find_all: bool,
}
