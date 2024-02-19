mod models;
mod parser;

use models::Program;
use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        return Err(Box::<dyn Error>::from("Usage: taskmaster <config_file>"));
    }

    let file_content = fs::read_to_string(&args[0])
        .map_err(|err| format!("Impossible to read config file -> {err}"))?;

    let programs: Vec<Program> = serde_yaml::from_str(&file_content)
        .map_err(|err| format!("Error while parsing config file -> {err}"))?;

    println!("{programs:#?}");

    Ok(())
}
