use std::collections::HashMap;

use crate::formatter::CustomFormatArgs;
use dynfmt::{Format, PythonFormat};

mod formatter;
mod parser;

fn main() {
    let env_vars: HashMap<String, String> = std::env::vars().collect();
    let programs = parser::parse_config_file("config.yaml").expect("Config file parse failed");
    // println!("{programs:#?}");

    let prog = &programs[0];
    let fmt = PythonFormat.format(
        "%(process_name)s",
        CustomFormatArgs::new(prog, &env_vars, 0),
    );
    println!("{}", fmt.expect("Erjdkndnjror"));
}
