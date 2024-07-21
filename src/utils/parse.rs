use super::fs::get_alias_file;
use crate::commands::alias::Alias;
use std::{fs::read_to_string, io::Result};

/// Parse alias configuration and return Value
pub fn parse_alias_config() -> Result<Vec<Alias>> {
    let alias_file_path = get_alias_file()?;
    let alias_file_content = read_to_string(&alias_file_path)?;
    let alias_value: Vec<Alias> = serde_json::from_str(alias_file_content.as_str())?;
    Ok(alias_value)
}

/// Insert alias to the alias list
pub fn insert_alias(alias_object_to_insert: Alias, alias_list: &mut Vec<Alias>) {
    // A flag denoting if new alias object exists in the list
    let mut is_new = true;

    for alias_object in alias_list.iter_mut() {
        if alias_object.alias == alias_object_to_insert.alias {
            alias_object.path = alias_object_to_insert.path.clone();
            is_new = false; // The new alias object exists in the list;
        }
    }

    if is_new {
        alias_list.push(alias_object_to_insert);
    }
}
