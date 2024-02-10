mod custom;
mod default;
mod model;

pub use model::Program;

use std::fs;

pub fn parse_config_file(file: &str) -> Result<Vec<Program>, String> {
    let content = fs::read_to_string(file)
        .map_err(|err| format!("Impossible to read config file -> {err}"))?;
    serde_yaml::from_str::<Vec<Program>>(&content)
        .map_err(|err| format!("Error while parsing config file -> {err}"))
}
