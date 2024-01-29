use crate::parser::model::AutoRestart;
use crate::parser::model::LogFile;
use crate::parser::model::Signal;

pub struct Default;

impl Default {
    pub fn numprocs() -> u8 {
        1
    }

    pub fn autostart() -> bool {
        true
    }

    pub fn startsecs() -> u32 {
        1
    }

    pub fn stopsignal() -> Signal {
        Signal::TERM
    }

    pub fn stopwaitsecs() -> u32 {
        10
    }

    pub fn redirect_stderr() -> bool {
        false
    }

    pub fn stdout_logfile() -> LogFile {
        LogFile::AUTO
    }

    pub fn stderr_logfile() -> LogFile {
        LogFile::AUTO
    }

    pub fn autorestart() -> AutoRestart {
        AutoRestart::UNEXPECTED
    }

    pub fn exitcodes() -> Vec<i32> {
        vec![0]
    }

    pub fn startretries() -> u32 {
        3
    }
}
