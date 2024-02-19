#![warn(missing_docs)]

//! TODO: Add documentation

mod models;
mod parser;

use models::Program;
use parser::parse_config_file;
use std::env;

/// Entry point
///
/// TODO: Add documentation
fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        return Err("Usage: taskmaster <config_file>".into());
    }

    let programs = parse_config_file(&args[0])?;

    println!("{programs:#?}");

    Ok(())
}
