use std::fmt::Display;

use crate::time;

enum LogLevel {
    Info,
    Error,
}

pub struct Log {
    level: LogLevel,
    time: String,
    data: String,
}

impl Log {
    pub fn info(data: String) {
        let log = Log {
            level: LogLevel::Info,
            time: time::now(),
            data,
        };
        println!("{}", log)
    }
    pub fn error(data: String) {
        let log = Log {
            level: LogLevel::Error,
            time: time::now(),
            data,
        };
        println!("{}", log)
    }
}

impl Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.level {
            LogLevel::Info => write!(f, "{} info:{}", self.time, self.data),
            LogLevel::Error => write!(f, "{} error:{}", self.time, self.data),
        }
    }
}
