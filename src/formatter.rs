use crate::parser::Program;
use dynfmt::{Argument, FormatArgs};
use std::collections::HashMap;

pub struct CustomFormatArgs<'a> {
    program: &'a Program,
    env_vars: &'a HashMap<String, String>,
    process_num: u32,
}

impl<'a> CustomFormatArgs<'a> {
    pub fn new(
        program: &'a Program,
        env_vars: &'a HashMap<String, String>,
        process_num: u32,
    ) -> CustomFormatArgs<'a> {
        CustomFormatArgs {
            program,
            env_vars,
            process_num,
        }
    }
}

impl<'a> FormatArgs for CustomFormatArgs<'a> {
    fn get_key(&self, key: &str) -> Result<Option<Argument<'_>>, ()> {
        let prefix_env = "ENV_";

        if key.starts_with(prefix_env) {
            let value = self.env_vars.get(key);
            if let None = value {
                return Ok(None);
            }
            return Ok(Some(value.unwrap() as Argument));
        }

        if key.eq("process_num") {
            return Ok(Some(&self.process_num as Argument));
        }

        let value = self.program.get_value(key);
        if value == None {
            return Ok(None);
        }
        return Ok(Some(value.unwrap() as Argument));
    }
}
