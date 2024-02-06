use std::{
    io::Result,
    process::{self, Command, Stdio},
};

fn get_tmux_sessions() -> (Vec<String>, bool) {
    let output = Command::new("tmux")
        .arg("list-sessions")
        .output()
        .expect("failed to execute process");

    let output = output.stdout;
    let output = String::from_utf8_lossy(&output);

    let mut is_attached = false;

    let sessions = output
        .lines()
        .map(|s| {
            let s_string = s.to_string();
            let s_split = s_string.split(":").collect::<Vec<_>>();

            if s.contains("(attached)") {
                is_attached = true;
            }

            s_split[0].to_string()
        })
        .collect::<Vec<String>>();

    (sessions, is_attached)
}

pub fn goto_session(dirpath: &String) -> Result<()> {
    let replacable = vec![".", ":"];

    let session_name = dirpath
        .chars()
        .map(|c| {
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

    let (sessions, is_attached) = get_tmux_sessions();

    let is_already_open = sessions.contains(&session_name);

    if is_attached {
        let output = if is_already_open {
            Command::new("sh")
                .arg("-c")
                .arg(format!("tmux switch-client -t \"{}\"", session_name))
                .output()
        } else if !is_already_open {
            Command::new("sh")
            .arg("-c")
            .arg(format!("tmux new -s \"{session_name}\" -c \"{session_name}\" -d && tmux switch-client -t \"{session_name}\""))
            .output()
        } else {
            unreachable!()
        }?;
        if !output.status.success() {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            process::exit(1);
        }
    } else {
        let mut child = if is_already_open {
            Command::new("sh")
                .arg("-c")
                .arg(format!("tmux attach -t \"{session_name}\""))
                .stdout(Stdio::inherit())
                .spawn()
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "tmux new -s \"{session_name}\" -c \"{session_name}\""
                ))
                .stdout(Stdio::inherit())
                .spawn()
        }?;

        let status = child.wait()?;
        if !status.success() {
            process::exit(status.code().unwrap_or(1));
        }
    }

    Ok(())
}
