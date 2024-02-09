use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

use dirs::config_dir;
use serde_yaml::{to_string, Value};

use crate::tmux::get_tmux_start_dir;

#[derive(Debug)]
pub struct Alias {
    pub key: String,
    pub value: String,
}

pub fn get_aliases() -> std::io::Result<Vec<Alias>> {
    let alias_config = get_alias_config()?;

    let mut aliases = Vec::new();

    if let Value::Mapping(map) = alias_config {
        for (k, v) in &map {
            if let Value::String(key) = k {
                if let Value::String(value) = v {
                    aliases.push(Alias {
                        key: key.to_string(),
                        value: value.to_string(),
                    })
                }
            }
        }
    }

    println!("{:?}", aliases);

    Ok(aliases)
}

fn get_alias_config_file() -> std::io::Result<File> {
    if config_dir().is_none() {
        eprintln!("No config dir found");
        std::process::exit(1);
    }
    let config_dir = config_dir().unwrap().join("ssm");
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir)?;
    }

    let alias_file_path = config_dir.join("alias.yaml");
    if !alias_file_path.exists() {
        File::create(&alias_file_path)?;
    }

    let alias_file = match OpenOptions::new()
        .read(true)
        .write(true)
        .open(&alias_file_path)
    {
        Ok(file) => file,
        Err(_) => {
            unreachable!();
        }
    };

    Ok(alias_file)
}

fn get_alias_config() -> std::io::Result<Value> {
    let mut alias_content = String::new();
    let mut alias_file = get_alias_config_file()?;
    alias_file.read_to_string(&mut alias_content)?;

    let alias_yaml: Value = match serde_yaml::from_str(&alias_content) {
        Ok(alias) => alias,
        Err(_) => {
            handle_parse_failed();
            std::process::exit(1);
        }
    };

    Ok(alias_yaml)
}

fn get_alias_from_args() -> String {
    let args = std::env::args().collect::<Vec<String>>();
    let command_args =
        crate::Commands::get_commandargs_by_enum(crate::commands::CommandEnum::SetAlias);

    if command_args.is_none() {
        eprintln!("No command found");
        std::process::exit(1);
    }
    let command_args = command_args.unwrap();

    let index = args
        .iter()
        .position(|arg| command_args.args.contains(arg))
        .unwrap_or_else(|| {
            eprintln!("No args found");
            std::process::exit(1);
        });

    if index + 1 >= args.len() {
        eprintln!("No alias found");
        std::process::exit(1);
    }

    let alias: String = args[index + 1].to_string();

    alias
}

fn add_alias(key: &String, value: &String, alias_config: &mut Value) {
    match alias_config {
        Value::Mapping(map) => {
            for (k, v) in map.clone().iter_mut() {
                if v == &Value::String(value.to_string()) {
                    // remove key
                    map.remove(k);
                    break;
                }
            }
            map.insert(
                serde_yaml::Value::String(key.to_string()),
                serde_yaml::Value::String(value.to_string()),
            );
        }
        Value::Null => {
            *alias_config = serde_yaml::Value::Mapping(
                vec![(
                    serde_yaml::Value::String(key.to_string()),
                    serde_yaml::Value::String(value.to_string()),
                )]
                .into_iter()
                .collect(),
            );
        }
        _ => {
            eprintln!("Error adding alias");
            std::process::exit(1);
        }
    }
}

fn handle_parse_failed() {
    println!("Error parsing alias config file.");
    println!("Do you want to reset the file? (y/n)");
    let mut input = String::new();

    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            if input.trim() == "y" {
                match get_alias_config_file() {
                    Ok(_) => {}
                    Err(_) => {
                        eprintln!("Error resetting file");
                        std::process::exit(1);
                    }
                };
            }
        }
        Err(_) => {
            eprintln!("Error reading input");
            std::process::exit(1);
        }
    };

    std::process::exit(1);
}

pub fn set_alias() -> std::io::Result<()> {
    let new_alias = get_alias_from_args();
    let mut alias_config_file = get_alias_config_file()?;
    let mut alias_config = get_alias_config()?;
    let tmux_dir = match get_tmux_start_dir()?.to_str() {
        Some(dir) => dir.to_string(),
        None => {
            eprintln!("Error getting tmux dir");
            std::process::exit(1);
        }
    };

    add_alias(&new_alias, &tmux_dir, &mut alias_config);

    let alias_config_str = match to_string(&alias_config) {
        Ok(alias) => alias,
        Err(_) => {
            eprintln!("Error converting to string");
            std::process::exit(1);
        }
    };

    alias_config_file.write_all(alias_config_str.as_bytes())?;

    Ok(())
}
