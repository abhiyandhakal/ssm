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
    size: String,
    panes: Vec<Pane>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Session {
    name: String,
    start_dir: String,
    windows: Vec<Window>,
}

impl Session {
    fn get_windows(session_name: &String) -> Result<Vec<Window>> {
        let output = Command::new("tmux")
            .arg("list-windows")
            .arg("-t")
            .arg(session_name)
            .output()?;
        let output = String::from_utf8_lossy(&output.stdout);
        let output = output.trim();
        let mut windows: Vec<Window> = vec![];

        for line in output.lines() {
            let line = line.trim();
            let line = line.split_whitespace().collect::<Vec<_>>();

            // let index
            let index = line[0].split(":").collect::<Vec<_>>()[0]
                .parse::<i32>()
                .unwrap_or_else(|_| {
                    eprintln!("Failed to parse window index");
                    std::process::exit(1);
                });
            // active window
            let active = line[1].contains("*");
            // let size
            let mut chars = line[4].chars();
            chars.next();
            chars.next_back();
            let size = chars.as_str().to_string();

            windows.push(Window {
                index,
                active,
                size,
                panes: vec![],
            });
        }

        dbg!(&windows);

        Ok(windows)
    }

    fn new(session_name: &String) -> Result<Session> {
        let start_dir = get_tmux_start_dir()?.to_string_lossy().to_string();
        let _windows = Self::get_windows(session_name)?;

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
    let _session: Value = serde_yaml::from_str(session.as_str()).unwrap_or_else(|_| {
        eprintln!("Failed to deserialize session");
        std::process::exit(1);
    });

    Ok(())
}
