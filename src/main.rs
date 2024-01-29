mod parser;
use crate::parser::Program;

use std::fs;

fn main() {
    let config = fs::read_to_string("config.yaml").expect("Impossible to read config file");
    println!("{config}");

    match serde_yaml::from_str::<Vec<Program>>(&config) {
        Ok(res) => println!("{res:#?}"),
        Err(err) => println!("{err:#?}"),
    }
}
