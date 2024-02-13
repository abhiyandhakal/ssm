use crate::tmux::{get_current_session, get_tmux_start_dir};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::{io::Result, process::Command};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Pane {
    index: i32,
    active: bool,
    command: String,
    working_dir: String,
    size: String,
    algorithm: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Window {
    index: i32,
    active: bool,
    panes: Vec<Pane>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Session {
    name: String,
    start_dir: String,
    windows: Vec<Window>,
}

impl Session {
    fn get_windows(session_name: &String) -> Result<()> {
        let output = Command::new("tmux")
            .arg("list-windows")
            .arg("-t")
            .arg(session_name)
            .output()?;
        let output = String::from_utf8_lossy(&output.stdout);
        let output = output.trim();
        println!("{}", output);

        Ok(())
    }

    fn new(session_name: &String) -> Result<Session> {
        let start_dir = get_tmux_start_dir()?.to_string_lossy().to_string();

        Ok(Session {
            name: session_name.to_string(),
            start_dir,
            windows: Vec::new(),
        })
    }
}

pub fn save_session() -> Result<()> {
    let platform = std::env::consts::OS;
    let _state_dir = match match platform {
        "linux" => dirs::state_dir(),
        _ => dirs::cache_dir(),
    } {
        Some(dir) => dir,
        None => {
            eprintln!("No cache or state dir found");
            std::process::exit(1);
        }
    }
    .join("ssm");

    let session_name = get_current_session()?;
    let session = Session::new(&session_name)?;
    let session = serde_yaml::to_string(&session).unwrap_or_else(|_| {
        eprintln!("Failed to serialize session");
        std::process::exit(1);
    });
    let session: Value = serde_yaml::from_str(session.as_str()).unwrap_or_else(|_| {
        eprintln!("Failed to deserialize session");
        std::process::exit(1);
    });

    dbg!(session);

    Ok(())
}
