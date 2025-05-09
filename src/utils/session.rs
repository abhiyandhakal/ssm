use serde::{Deserialize, Serialize};
use std::{
    fs::read_to_string,
    io::{Error, Result},
    path::PathBuf,
};

use crate::utils::command::execute_command;

use super::{fs::get_state_dir, parse::parse_sessions_in_files, utils::remove_first_and_last};

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
       [pane_index] [width] [height] [window_width] [window_height] [pane_PID] [current_path] [is_pane_active]
    */
    let input_str = format!("tmux list-panes -F '#{{pane_index}} #{{pane_width}} #{{pane_height}} #{{window_width}} #{{window_height}} #{{pane_pid}} #{{pane_current_path}} #{{?pane_active,(active),}}' -t {session_name}:{window_index}");
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

        // let pane size in percentage (format: [width_percent]x[height_percent])
        let width_percent: f32 =
            line[1].parse::<f32>().unwrap_or(1.0) / line[3].parse::<f32>().unwrap_or(1.0) * 100.0;
        let height_percent =
            line[2].parse::<f32>().unwrap_or(1.0) / line[4].parse::<f32>().unwrap_or(1.0) * 100.0;
        let size = format!("{width_percent}x{height_percent}");

        // let command
        let command_pid = execute_command(format!("ps -o pid= --ppid {}", line[5]))?;
        let command = execute_command(format!("ps -o command -p {command_pid}"))?;
        let command = command.split('\n').collect::<Vec<_>>();
        let command = command[command.len() - 1];

        panes.push(Pane {
            index,
            active: line.last().is_some_and(|f| f == &"(active)"),
            size,
            command: command.to_string(),
            working_dir: line[line.len() - 2].to_string(),
        })
    }

    Ok(panes)
}

/// Restores the session if saved.
/// *Note:* Create the session first, and then pass the session name as an argument.
pub fn restore_session(session_name: &String) -> Result<()> {
    let parsed_sessions_saved = parse_sessions_in_files()?;
    let mut session_saved = None;

    for (_, session_saved_in_file) in parsed_sessions_saved {
        if session_name == &session_saved_in_file.name {
            session_saved = Some(session_saved_in_file);
            break;
        }
    }

    if session_saved.is_none() {
        return Err(Error::new(
            std::io::ErrorKind::NotFound,
            "Session hasn't been saved to restore.",
        ));
    }

    let session_saved = session_saved.unwrap();
    println!("{:#?}", session_saved);

    for window in session_saved.windows {
        // Create window
        execute_command(format!(
            "tmux new-window -t {}:{}",
            session_name, window.index
        ))?;

        // Set active window
        if window.active {
            execute_command(format!(
                "tmux select-window {}:{}",
                session_name, window.index
            ))?;
        }

        // Create panes in the specific windows
        // TODO: Restore sizes and orientation of the panes
        let mut pane_count = 0;
        let mut start_index_saved_difference = 0;
        for pane in &window.panes {
            execute_command(format!(
                "tmux split-window -t {}:{}",
                session_name, window.index
            ))?;

            if pane_count == 0 {
                let pane_indices = execute_command(format!(
                    "tmux list-panes -t {}:{}",
                    session_name, window.index
                ))?
                .split('\n')
                .map(|f| f.trim().parse::<i32>().unwrap_or(0))
                .collect::<Vec<_>>();

                // Make sure the panes to target have correct indices by
                // gauging the difference between the saved and the created panes
                if pane_indices.len() != 0 {
                    start_index_saved_difference = pane_indices[0] - pane.index;
                }
                pane_count += 1;
            }

            // Set active pane
            if pane.active {
                execute_command(format!(
                    "tmux select-pane -t {}:{}.{}",
                    session_name, window.index, pane.index
                ))?;
            }
        }

        // Restore commands in the panes
        for pane in window.panes {
            execute_command(format!(
                "tmux send-keys -t {}:{}.{} '{}' C-m",
                session_name,
                window.index,
                pane.index + start_index_saved_difference,
                pane.command
            ))?;
        }
    }

    Ok(())
}
