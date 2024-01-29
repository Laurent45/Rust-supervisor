use crate::parser::default::Default;

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[allow(dead_code)] // TODO: Remove
pub struct Program {
    name: String,
    command: String,
    #[serde(default = "Default::numprocs")]
    numprocs: u8, // Note that if numprocs > 1, the process_name expression must include. %(process_num)s
    #[serde(default = "Default::autostart")]
    autostart: bool,
    #[serde(default = "Default::autorestart")]
    autorestart: AutoRestart,
    #[serde(default = "Default::exitcodes")]
    exitcodes: Vec<i32>,
    #[serde(default = "Default::startsecs")]
    startsecs: u32,
    #[serde(default = "Default::startretries")]
    startretries: u32,
    #[serde(default = "Default::stopsignal")]
    stopsignal: Signal,
    #[serde(default = "Default::stopwaitsecs")]
    stopwaitsecs: u32,
    #[serde(default = "Default::redirect_stderr")]
    redirect_stderr: bool,
    #[serde(default = "Default::stdout_logfile")]
    stdout_logfile: LogFile,
    #[serde(default = "Default::stderr_logfile")]
    stderr_logfile: LogFile,
    environement: Option<HashMap<String, String>>,
    directory: Option<String>,
    umask: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum LogFile {
    AUTO,
    NONE,
    PATH(String),
}

#[derive(Deserialize, Serialize, Debug)]
pub enum AutoRestart {
    UNEXPECTED,
    TRUE,
    FALSE,
}

#[derive(Debug)]
pub enum Signal {
    TERM,
    KILL,
}
