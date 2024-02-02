use std::process::Command;

fn get_tmux_sessions() -> (Vec<String>, String) {
    let output = Command::new("tmux")
        .arg("list-sessions")
        .output()
        .expect("failed to execute process");

    let output = output.stdout;
    let output = String::from_utf8_lossy(&output);

    let sessions = output
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    (sessions, output.to_string())
}

pub fn goto_session(dirpath: &String) {
    let replacable = vec!["."];

    let session_name = dirpath
        .chars()
        .map(|c| {
            println!("'{c}'");
            if replacable.contains(&c.to_string().as_str()) {
                "_".to_string()
            } else if c == '\\' {
                "\\\\".to_string()
            } else {
                c.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("");

    // let (sessions, active) = get_tmux_sessions();
    //
    // println!("sessions: {:?}", sessions);

    println!("session_name: {}", session_name);
}
