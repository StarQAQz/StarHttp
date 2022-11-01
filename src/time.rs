use std::{sync::RwLock, time};

use crate::config::MyConfig;

#[derive(Debug)]
struct Date {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    second: i32,
    timestamp: i32,
}

impl Date {
    fn update(&mut self) {
        let time_sec = time::SystemTime::now()
            .duration_since(time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i32
            + MyConfig::new().timezone * 60 * 60;
        let mut temp;
        //计算年
        loop {
            if (self.year % 4 == 0 && self.year % 100 != 0) || self.year % 400 == 0 {
                temp = 366 * 24 * 60 * 60;
            } else {
                temp = 365 * 24 * 60 * 60;
            }
            if self.timestamp + temp > time_sec {
                break;
            }
            self.timestamp += temp;
            self.year += 1;
        }
        //计算月份
        let month_days;
        if (self.year % 4 == 0 && self.year % 100 != 0) || self.year % 400 == 0 {
            month_days = LEAP_MONTH_DAYS;
        } else {
            month_days = MONTH_DAYS;
        }
        for days in month_days {
            temp = days * 24 * 60 * 60;
            if self.timestamp + temp > time_sec {
                break;
            }
            self.timestamp += temp;
            self.month += 1;
        }
        for days in MONTH_DAYS {
            temp = days * 24 * 60 * 60;
            if self.timestamp + temp > time_sec {
                break;
            }
            self.timestamp += temp;
            self.month += 1;
        }
        //计算天数
        while self.timestamp + 24 * 60 * 60 < time_sec {
            self.timestamp += 24 * 60 * 60;
            self.day += 1;
        }
        //计算小时
        while self.timestamp + 60 * 60 <= time_sec {
            self.timestamp += 60 * 60;
            self.hour += 1;
        }
        //计算分钟
        while self.timestamp + 60 <= time_sec {
            self.timestamp += 60;
            self.minute += 1;
        }
        //计算秒
        self.second = time_sec - self.timestamp;
    }
}

impl ToString for Date {
    fn to_string(&self) -> String {
        format!(
            "{}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

static MONTH_DAYS: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
static LEAP_MONTH_DAYS: [i32; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
static DATE: RwLock<Date> = RwLock::new(Date {
    year: 1970,
    month: 1,
    day: 1,
    hour: 0,
    minute: 0,
    second: 0,
    timestamp: 0,
});

pub fn now() -> String {
    if let Ok(mut date) = DATE.try_write() {
        date.update();
    }
    DATE.read().unwrap().to_string()
}

#[cfg(test)]
mod time_test {
    use std::{thread, time::Duration};

    use crate::time::{now, DATE};

    #[test]
    fn get_now() {
        loop {
            println!("{}", now());
            println!("DATE:{:?}", DATE);
            thread::sleep(Duration::from_secs(1));
        }
    }
}
