use std::{io::Result, path::PathBuf};

use crate::commands::alias::Alias;

/// Get ssm's config directory
pub fn get_config_dir() -> Result<PathBuf> {
    let config_dir = dirs::config_dir();

    if config_dir.is_none() {
        return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
    };

    let config_dir = config_dir.unwrap();

    let ssm_config_dir = config_dir.join("ssm");

    // Create ssm directory if doesn't exist
    if !ssm_config_dir.is_dir() {
        std::fs::create_dir_all(&ssm_config_dir)?;
    }

    Ok(ssm_config_dir)
}

/// Get path of the file where aliases are saved
pub fn get_alias_file() -> Result<PathBuf> {
    let config_dir = get_config_dir()?;
    let alias_file = config_dir.join("alias.json");

    // Create alias file if it doesn't exist
    if !alias_file.is_file() {
        std::fs::write(&alias_file, "[]")?;
    }

    Ok(alias_file)
}

/// Save the input alias list to the config file
pub fn save_alias_list_to_file(alias_list: &Vec<Alias>) -> Result<()> {
    let alias_list_str = serde_json::to_string(alias_list)?;
    let alias_file = get_alias_file()?;

    std::fs::write(alias_file, alias_list_str)?;

    Ok(())
}
