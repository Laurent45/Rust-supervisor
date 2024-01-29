use crate::parser::model::Signal;

use serde::de::Error;
use serde::Deserialize;
use serde::Deserializer;

impl<'de> Deserialize<'de> for Signal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.parse::<u8>() {
            Ok(signum) => Signal::from_u8(signum).map_or(
                Err(Error::custom(format!(
                    "{signum} is not a valid signal number"
                ))),
                |signum| Ok(signum),
            ),
            Err(_) => Signal::from_str(&value).map_or(
                Err(Error::custom(format!("{value} is not an unix signal"))),
                |signal| Ok(signal),
            ),
        }
    }
}

// TODO: Find a better way to handle signal str to signal number
impl Signal {
    pub fn from_str(value: &str) -> Option<Signal> {
        match value {
            "TERM" => Some(Signal::TERM),
            "KILL" => Some(Signal::KILL),
            _ => None,
        }
    }

    pub fn from_u8(value: u8) -> Option<Signal> {
        match value {
            15 => Some(Signal::TERM),
            9 => Some(Signal::KILL),
            _ => None,
        }
    }
}
