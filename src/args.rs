use clap::Parser;

/// Smart Session Manager for Tmux
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path of the directory or the alias you have saved as
    #[arg(index = 1)]
    pub path_or_alias: Option<String>,

    #[arg(short, long)]
    pub set_alias: Option<String>,
}
