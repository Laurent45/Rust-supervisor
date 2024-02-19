use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[allow(dead_code)] // TODO: Remove
pub struct Program {
    pub name: String,
    pub command: String,
    #[serde(default = "numprocs")]
    pub numprocs: u8, // Note that if numprocs > 1, the process_name expression must include. %(process_num)s
    #[serde(default = "autostart")]
    pub autostart: bool,
    #[serde(default = "autorestart")]
    pub autorestart: AutoRestart,
    #[serde(default = "exitcodes")]
    pub exitcodes: Vec<i32>,
    #[serde(default = "startsecs")]
    pub startsecs: u32,
    #[serde(default = "startretries")]
    pub startretries: u32,
    #[serde(default = "stopsignal")]
    pub stopsignal: Signal,
    #[serde(default = "stopwaitsecs")]
    pub stopwaitsecs: u32,
    #[serde(default = "redirect_stderr")]
    pub redirect_stderr: bool,
    #[serde(default = "stdout_logfile")]
    pub stdout_logfile: LogFile,
    #[serde(default = "stderr_logfile")]
    pub stderr_logfile: LogFile,
    pub environement: Option<HashMap<String, String>>,
    pub directory: Option<String>,
    pub umask: Option<u32>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum LogFile {
    AUTO,
    NONE,
    PATH(String),
}

#[derive(Deserialize, Debug)]
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

fn numprocs() -> u8 {
    1
}

fn autostart() -> bool {
    true
}

fn startsecs() -> u32 {
    1
}

fn stopsignal() -> Signal {
    Signal::TERM
}

fn stopwaitsecs() -> u32 {
    10
}

fn redirect_stderr() -> bool {
    false
}

fn stdout_logfile() -> LogFile {
    LogFile::AUTO
}

fn stderr_logfile() -> LogFile {
    LogFile::AUTO
}

fn autorestart() -> AutoRestart {
    AutoRestart::UNEXPECTED
}

fn exitcodes() -> Vec<i32> {
    vec![0]
}

fn startretries() -> u32 {
    3
}
