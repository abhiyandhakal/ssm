use super::parse::parse_alias_config;
use crate::commands::alias::Alias;

/// Search if the input exists in alias list. If exists, return the alias-path pair
pub fn browse_alias<T: AsRef<str>>(input: T) -> Option<Alias> {
    let input = input.as_ref();
    let alias_config_vec = match parse_alias_config() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };

    // Return the alias object if an alias matches
    for alias_obj in &alias_config_vec {
        if alias_obj.alias == input {
            return Some(alias_obj.clone());
        }

        if let Ok(path) = std::fs::canonicalize(input) {
            let path = path.to_string_lossy().to_string();
            if alias_obj.path == path {
                return Some(alias_obj.clone());
            }
        }
    }

    None
}

/// Removes the first and last character of the string
pub fn remove_first_and_last<T: AsRef<str>>(input: T) -> String {
    let input = input.as_ref();
    let input_trimmed = &input[1..input.len() - 1];

    String::from(input_trimmed)
}
