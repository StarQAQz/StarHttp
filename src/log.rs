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
        eprintln!("{}", log)
    }
}

impl Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.level {
            LogLevel::Info => write!(f, "{} INFO: {}", self.time, self.data),
            LogLevel::Error => write!(f, "{} ERROR: {}", self.time, self.data),
        }
    }
}

#[macro_export]
macro_rules! log_info {
    ($fmt:expr) => {$crate::log::Log::info(format!($fmt))};
    ($fmt:expr,$($arg:tt)*)=>{$crate::log::Log::info(format!($fmt,($($arg)*)))};
}

#[macro_export]
macro_rules! log_error {
    ($fmt:expr) => {$crate::log::Log::error(format!($fmt))};
    ($fmt:expr,$($arg:tt)*)=>{$crate::log::Log::error(format!($fmt,($($arg)*)))};
}

#[cfg(test)]
mod log_test {
    #[test]
    fn log_info_test() {
        log_info!("hello world");
        log_info!("hello world{}", "!");
    }
}
