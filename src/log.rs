use std::{
    fmt::{self, Display},
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
        //     time.as_secs()
        // };
        // match self.level {
        //     LogLevel::info => write!(f, "{} info:{}", self.time, self.data),
        //     LogLevel::error => write!(f, "{} error:{}", self.time, self.data),
        // }
    }
}

fn get_date(time_sec: u64) -> String {
    let ago = 0;
    let year = 1970;
    let temp = 0;
    loop {
        if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
            temp = 366 * 24 * 60 * 60;
        } else {
            temp = 365 * 24 * 60 * 60;
        }
        if ago + temp > time_sec {
            break;
        }
        ago += temp;
        year += 1;
    }
    let month_days = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
        month_days[1] = 29;
    }
    let month = 1;
    for days in month_days {
        temp = days * 24 * 60 * 60;
        if ago + temp > time_sec {
            break;
        }
        ago += temp;
        month += 1;
    }
    let day = 1;
    while ago + 24 * 60 * 60 < time_sec {
        ago += 24 * 60 * 60;
        day += 1;
    }
    let hour = 0;
    while ago + 60 * 60 < time_sec {
        ago += 60 * 60;
        hour += 1;
    }
    let minute = 0;
    while ago + 60 < time_sec {
        ago += 60;
        minute += 1;
    }
    let second = time_sec - ago;
    format!(
        "{}-{:02}-{:02} {:02}:{:02}:{:02}",
        year, month, day, hour, minute, second,
    )
}
