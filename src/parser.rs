mod signal_parser;

use crate::Program;
use std::fs;

/// Take a yaml file path and parse the content to vector of Program
///
/// Try to open the file given on parameter and parse the content to
/// a vector of Program. An error may be return if file is not found or
/// if file a not parseable
///
/// # Exemples
///
/// ```
/// let programs = parse_config_file("config.yaml");
/// ```
///
/// [`Program`]: ./models/program.rs
pub fn parse_config_file(file_path: &str) -> Result<Vec<Program>, String> {
    match fs::read_to_string(file_path) {
        Ok(file_content) => serde_yaml::from_str(&file_content)
            .map_err(|err| format!("Error while parsing config file -> {err:?}",)),
        Err(err) => Err(format!("Impossible to read config file -> {err:?}")),
    }
}
