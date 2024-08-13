use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, io::Result, path::PathBuf};

use crate::utils::command::execute_command;

use super::{fs::get_state_dir, utils::remove_first_and_last};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Point {
    x: f64,
    y: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Pane {
    index: i32,
    active: bool,
    command: String,
    working_dir: String,
    size: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Window {
    index: i32,
    active: bool,
    size: String,
    panes: Vec<Pane>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Session {
    pub name: String,
    pub start_dir: String,
    pub windows: Vec<Window>,
}

/// Get the tmux windows details and the panes within
pub fn get_windows(session_name: &String) -> Result<Vec<Window>> {
    let output = execute_command(format!("tmux list-windows -t {session_name}"))?;
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
            panes: get_panes(session_name, index)?,
        });
    }

    Ok(windows)
}

/// Get the details about a pane in a window of a tmux session
pub fn get_panes(session_name: &String, window_index: i32) -> Result<Vec<Pane>> {
    /*
       List panes of the provided session_name and window_index
       Format:
       [pane_index] [width]x[height] [pane_PID] [current_command] [current_path] [is_pane_active]
    */
    let input_str = format!("tmux list-panes -F '#{{pane_index}} #{{pane_width}}x#{{pane_height}} #{{pane_pid}} #{{pane_current_command}} #{{pane_current_path}} #{{?pane_active,(active),}}' -t {session_name}:{window_index}");
    let output = execute_command(input_str)?;
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
        let command_pid = execute_command(format!("ps -o pid= --ppid {}", line[2]))?;
        let command = execute_command(format!("ps -o command -p {command_pid}"))?;
        let command = command.split('\n').collect::<Vec<_>>();
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

// /// To save the session, get a new file name if not saved previously, or return a previously saved file
// pub fn get_session_save_filename() -> Result<PathBuf> {
//     let state_dir = get_state_dir()?;
//     let files_read = std::fs::read_dir(state_dir)?;
//     let mut files = vec![];
//     for file in files_read {
//         let file = file?.path();
//         if file.is_file() {
//             files.push(file)
//         }
//     }
//
//     for file in files {
//         let file_content = read_to_string(file)?;
//         let file_content = serde_json::from_str(&file_content);
//     }
//
//     Ok(PathBuf::new())
// }
