use std::{
    fmt::Display,
    time::{self, SystemTime},
};

enum LogLevel {
    info,
    error,
}

pub struct Log {
    level: LogLevel,
    time: SystemTime,
    data: String,
}

impl Log {
    fn new(level: LogLevel, data: String) -> Log {
        return Log {
            level,
            time: SystemTime::now(),
            data,
        };
    }
    pub fn info(data: String) {
        let log = Self::new(LogLevel::info, data);
        println!("{}", log)
    }
}

impl Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
        // if let Ok(time) = self.time.duration_since(time::UNIX_EPOCH) {

        // };
        // match self.level {
        //     LogLevel::info => write!(f, "{} info:{}", self.time, self.data),
        //     LogLevel::error => write!(f, "{} error:{}", self.time, self.data),
        // }
    }
}
