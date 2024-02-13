use crate::{
    tmux::{get_current_session, get_tmux_start_dir},
    utils::remove_first_and_last,
};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::{
    fs::{File, OpenOptions},
    io::{Read, Result, Write},
    process::Command,
};

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
                .arg("--ppid")
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
            let command = command.trim().split('\n').collect::<Vec<_>>();
            let command = command[command.len() - 1];

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

        Ok(windows)
    }

    fn new(session_name: &String) -> Result<Session> {
        let start_dir = get_tmux_start_dir()?.to_string_lossy().to_string();
        Ok(Session {
            name: session_name.to_string(),
            start_dir,
            windows: Self::get_windows(session_name)?,
        })
    }
}

fn check_session_exists_in_state(
    state_dir: &std::path::PathBuf,
    session_name: &String,
) -> Result<Option<String>> {
    let files = std::fs::read_dir(&state_dir)?;
    for file in files {
        let file = file?;
        let path = file.path();

        // check if file is a file
        if !file.file_type()?.is_file() {
            continue;
        }

        // check if file is a yaml file
        let ext = path.extension().unwrap_or_default();
        if ext != "yaml" && ext != "yml" {
            continue;
        }
        let mut file = OpenOptions::new().read(true).open(&path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        let buffer: Value = serde_yaml::from_str(buffer.as_str()).unwrap_or_else(|_| {
            eprintln!("Failed to deserialize session");
            std::process::exit(1);
        });

        if buffer["name"] == *session_name {
            let filename = path.file_name().unwrap_or_default();
            let filename = filename.to_string_lossy().to_string();
            return Ok(Some(filename));
        }
    }

    Ok(None)
}

pub fn save_session() -> Result<()> {
    let platform = std::env::consts::OS;
    let args: Vec<String> = std::env::args().collect();
    let state_dir = match match platform {
        "linux" => dirs::state_dir(),
        _ => dirs::cache_dir(),
    } {
        Some(dir) => dir,
        None => {
            eprintln!("No cache or state dir found");
            std::process::exit(1);
        }
    }
    .join("ssm")
    .join("sessions");

    let session_name = if args.len() > 2 {
        args[2].to_string()
    } else {
        get_current_session()?
    };

    // check if session exists to save
    let output = Command::new("tmux")
        .arg("has-session")
        .arg("-t")
        .arg(&session_name)
        .output()?;
    if !output.status.success() {
        eprintln!("Session does not exist");
        std::process::exit(1);
    }

    let session = Session::new(&session_name)?;
    let session = serde_yaml::to_string(&session).unwrap_or_else(|_| {
        eprintln!("Failed to serialize session");
        std::process::exit(1);
    });
    std::fs::create_dir_all(&state_dir)?;

    // check if session exists
    let session_exists = check_session_exists_in_state(&state_dir, &session_name)?;
    if let Some(file_name) = session_exists {
        let file = state_dir.join(file_name);
        let mut file = OpenOptions::new().write(true).open(&file)?;
        file.write_all(session.as_bytes())?;

        println!("Session saved");
        return Ok(());
    }
    let unique_id = uuid::Uuid::new_v4();
    let file = state_dir.join(format!("{unique_id}.yaml"));
    let mut file = OpenOptions::new().write(true).create(true).open(&file)?;
    file.write_all(session.as_bytes())?;

    println!("Session saved");
    Ok(())
}
