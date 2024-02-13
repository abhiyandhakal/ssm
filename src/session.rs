use crate::{
    tmux::{get_current_session, get_tmux_start_dir},
    utils::remove_first_and_last,
};
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
    fn get_panes(session_name: &String, window_index: i32) -> Result<Vec<Pane>> {
        let output = Command::new("tmux")
            .arg("list-panes")
            .arg("-F")
            .arg("#{pane_index} #{pane_width}x#{pane_height} #{pane_pid} #{pane_current_command} #{pane_current_path} #{?pane_active,(active),}")
            .arg("-t")
            .arg(format!("{}:{}", session_name, window_index))
            .output()?;

        let output = String::from_utf8_lossy(&output.stdout);
        let output = output.trim();
        println!("{}", output);
        let mut panes: Vec<Pane> = vec![];

        for line in output.lines() {
            let line = line.trim();
            let line = line.split_whitespace().collect::<Vec<_>>();
            // let index
            let index = line[0].parse::<i32>().unwrap_or_else(|_| {
                eprintln!("Failed to parse pane index");
                std::process::exit(1);
            });
            // let command
            let output = Command::new("ps")
                .arg("-o")
                .arg("pid=")
                .arg(line[2])
                .output()?;
            let command_pid = String::from_utf8_lossy(&output.stdout);
            let command_pid = command_pid.trim();
            let output = Command::new("ps")
                .arg("-o")
                .arg("command")
                .arg("-p")
                .arg(command_pid)
                .output()?;
            let command = String::from_utf8_lossy(&output.stdout);
            let command = command.trim().lines().next().unwrap_or_else(|| {
                eprintln!("Failed to get command");
                std::process::exit(1);
            });

            panes.push(Pane {
                index,
                active: line.last().is_some_and(|f| f == &"(active)"),
                size: line[1].to_string(),
                command: command.to_string(),
                working_dir: line[4].to_string(),
            })
        }

        Ok(panes)
    }

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
            let size = remove_first_and_last(line[4]);

            windows.push(Window {
                index,
                active,
                size,
                panes: Self::get_panes(session_name, index)?,
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
