use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    #[derive(Debug)]
    static ref MONTHS: HashMap<&'static str, &'static str> = HashMap::from([
        ("january", "January"),
        ("february", "February"),
        ("march", "March"),
        ("april", "April"),
        ("may", "May"),
        ("june", "June"),
        ("july", "July"),
        ("august", "August"),
        ("september", "September"),
        ("october", "October"),
        ("november", "November"),
        ("december", "December"),
        ("jan", "January"),
        ("feb", "February"),
        ("mar", "March"),
        ("apr", "April"),
        ("jun", "June"),
        ("jul", "July"),
        ("aug", "August"),
        ("sep", "September"),
        ("oct", "October"),
        ("nov", "November"),
        ("dec", "December"),
        ("1", "January"),
        ("2", "February"),
        ("3", "March"),
        ("4", "April"),
        ("5", "May"),
        ("6", "June"),
        ("7", "July"),
        ("8", "August"),
        ("9", "September"),
        ("10", "October"),
        ("11", "November"),
        ("12", "December"),
        ("January", "1"),
        ("February", "2"),
        ("March", "3"),
        ("April", "4"),
        ("May", "5"),
        ("June", "6"),
        ("July", "7"),
        ("August", "8"),
        ("September", "9"),
        ("October", "10"),
        ("November", "11"),
        ("December", "12"),
    ]);
}

#[derive(Debug)]
struct Month {
    month_name: String,
    month_num: u8,
}

#[derive(Debug)]
pub struct DateTest {
    year: i32,
    month: Month,
    day: u8,
}

impl DateTest {
    pub fn new(year: i32, month: &str, day: u8) -> Option<DateTest> {
        let month = month.to_string();
        match MONTHS.get(month.to_lowercase().as_str()) {
            Some(month_name) => {
                let month = Month {
                    month_name: month_name.to_string(),
                    month_num: MONTHS.get(month_name).unwrap().parse::<u8>().unwrap(),
                };
                Some(DateTest {
                    year,
                    month,
                    day,
                })
            },
            _ => None,
        }
    }

    pub fn as_calendar_date_tuple(&self) -> (i32, String, u8) {
        (self.year, self.month.month_name.clone(), self.day)
    }

    pub fn as_calendar_date_string(&self) -> String {
        format!("{} {}, {}", self.month.month_name, self.day, self.year)
    }

    pub fn as_ordinal_date_tuple(&self) -> (i32, u8, u8) {
        (self.year, self.month.month_num, self.day)
    }

    pub fn as_ordinal_date_string(&self) -> String {
        format!("{} {}, {}", self.month.month_num, self.day, self.year)
    }
}

pub struct TimeTest {
    hour: u8,
    minute: u8,
}

impl TimeTest {
    pub fn new(hour: u8, minute: u8) -> Option<TimeTest> {
        if hour > 23 || minute > 59 {
            None
        } else {
            Some(TimeTest {
                hour,
                minute,
            })
        }
    }

    pub fn as_time_tuple(&self) -> (u8, u8) {
        (self.hour, self.minute)
    }

    pub fn as_24_hour_time_string(&self) -> String {
        let mut hour = self.hour.to_string();
        if hour.len() == 1 {
            hour = format!("0{}", hour);
        }
        let mut minute = self.minute.to_string();
        if minute.len() == 1 {
            minute = format!("0{}", minute);
        }
        format!("{}:{}", hour, minute)
    }

    pub fn as_12_hour_time_string(&self) -> String {
        let mut hour = self.hour.to_string();
        if hour.len() == 1 {
            hour = format!("0{}", hour);
        }
        let mut minute = self.minute.to_string();
        if minute.len() == 1 {
            minute = format!("0{}", minute);
        }

        let mut am_pm = "AM";
        if self.hour > 12 {
            am_pm = "PM";
            hour = (self.hour - 12).to_string();
        }
        format!("{}:{} {}", hour, minute, am_pm)
    }
}