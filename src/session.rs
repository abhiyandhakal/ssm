use std::{io::Result, process::Command};

use crate::tmux::get_current_session;

pub fn get_windows(session_name: &String) -> Result<()> {
    let output = Command::new("tmux")
        .arg("list-windows")
        .arg("-t")
        .arg(session_name)
        .output()?;

    println!("{:?}", output);

    Ok(())
}

pub fn save_session() -> Result<()> {
    let windows = get_windows(&get_current_session()?)?;

    Ok(())
}
