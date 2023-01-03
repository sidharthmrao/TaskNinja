use std::collections::HashMap;
use lazy_static::lazy_static;
use std::fmt;
use std::error::Error as StdError;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DateTimeError {
    InvalidYear,
    InvalidMonth,
    InvalidDay,
    InvalidHour,
    InvalidMinute,
    UnspecifiedDate,
    UnspecifiedTime,
}

impl fmt::Display for DateTimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DateTimeError::InvalidYear => write!(f, "Year out of bounds."),
            DateTimeError::InvalidMonth => write!(f, "Month out of bounds.m"),
            DateTimeError::InvalidDay => write!(f, "Day out of bounds."),
            DateTimeError::InvalidHour => write!(f, "Hour out of bounds."),
            DateTimeError::InvalidMinute => write!(f, "Minute out of bounds."),
            DateTimeError::UnspecifiedDate => write!(f, "Date not specified."),
            DateTimeError::UnspecifiedTime => write!(f, "Time not specified."),
        }
    }
}

impl StdError for DateTimeError {
    fn description(&self) -> &str {
        match self {
            DateTimeError::InvalidYear => "Year out of bounds.",
            DateTimeError::InvalidMonth => "Month out of bounds.",
            DateTimeError::InvalidDay => "Day out of bounds.",
            DateTimeError::InvalidHour => "Hour out of bounds.",
            DateTimeError::InvalidMinute => "Minute out of bounds.",
            DateTimeError::UnspecifiedDate => "Date not specified.",
            DateTimeError::UnspecifiedTime => "Time not specified.",
        }
    }
}

impl Serialize for DateTimeError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("DateTimeError", 1)?;
        state.serialize_field("error", &self.description())?;
        state.end()
    }
}

lazy_static! {
    #[derive(Debug)]
    static ref MONTHS_EXPAND: HashMap<&'static str, &'static str> = HashMap::from([
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
        ("may", "May"),
        ("jun", "June"),
        ("jul", "July"),
        ("aug", "August"),
        ("sep", "September"),
        ("oct", "October"),
        ("nov", "November"),
        ("dec", "December"),
        ("1", "January"),
        ("01", "January"),
        ("2", "February"),
        ("02", "February"),
        ("3", "March"),
        ("03", "March"),
        ("4", "April"),
        ("04", "April"),
        ("5", "May"),
        ("05", "May"),
        ("6", "June"),
        ("06", "June"),
        ("7", "July"),
        ("07", "July"),
        ("8", "August"),
        ("08", "August"),
        ("9", "September"),
        ("09", "September"),
        ("10", "October"),
        ("11", "November"),
        ("12", "December"),
    ]);
}

lazy_static! {
    #[derive(Debug)]
    static ref MONTH_TO_DAY: HashMap<&'static str, u8> = HashMap::from([
        ("January", 1),
        ("February", 2),
        ("March", 3),
        ("April", 4),
        ("May", 5),
        ("June", 6),
        ("July", 7),
        ("August", 8),
        ("September", 9),
        ("October", 10),
        ("November", 11),
        ("December", 12),
    ]);
}

lazy_static! {
    #[derive(Debug)]
    static ref DAY_LIMITS: HashMap<&'static str, u8> = HashMap::from([
        ("January", 31),
        ("February", 28),
        ("March", 31),
        ("April", 30),
        ("May", 31),
        ("June", 30),
        ("July", 31),
        ("August", 31),
        ("September", 30),
        ("October", 31),
        ("November", 30),
        ("December", 31),
    ]);
}

#[derive(Debug)]
struct Month {
    month_name: String,
    month_num: u8,
}

impl Serialize for Month {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Month", 2)?;
        state.serialize_field("month_name", &self.month_name)?;
        state.serialize_field("month_num", &self.month_num)?;
        state.end()
    }
}

#[derive(Debug)]
pub struct Date {
    year: i32,
    month: Month,
    day: u8,
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Date", 3)?;
        state.serialize_field("year", &self.year)?;
        state.serialize_field("month", &self.month)?;
        state.serialize_field("day", &self.day)?;
        state.end()
    }
}

impl Date {
    pub fn new(year: i32, month: &str, day: u8) -> Result<Date, DateTimeError> {
        let month = month.to_string();
        match MONTHS_EXPAND.get(month.to_lowercase().as_str()) {
            Some(month_name) => {
                if year < 0 {
                    Err(DateTimeError::InvalidYear)
                } else if day < 1 || day > DAY_LIMITS.get(month_name).unwrap().to_owned() as u8 {
                    Err(DateTimeError::InvalidDay)
                } else {
                    let month_num = MONTH_TO_DAY.get(month_name).unwrap().to_owned() as u8;
                    let month = Month {
                        month_name: month_name.to_string(),
                        month_num,
                    };

                    Ok(Date {
                        year,
                        month,
                        day,
                    })
                }
            }
            None => Err(DateTimeError::InvalidMonth),
        }
    }

    pub fn parse(date: &str) -> Result<Date, DateTimeError> {
        let date = date.to_string();
        let date = date.split("-").collect::<Vec<&str>>();
        if date.len() != 3 {
            return Err(DateTimeError::UnspecifiedDate);
        }

        let year = date[0].parse::<i32>().expect("Invalid year.");
        let month = date[1];
        let day = date[2].parse::<u8>().expect("Invalid day.");

        Date::new(year, month, day)
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

pub struct Time {
    hour: u8,
    minute: u8,
}

impl Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Time", 2)?;
        state.serialize_field("hour", &self.hour)?;
        state.serialize_field("minute", &self.minute)?;
        state.end()
    }
}

impl Time {
    pub fn new(hour: u8, minute: u8) -> Result<Time, DateTimeError> {
        if hour > 23 {
            Err(DateTimeError::InvalidHour)
        } else if minute > 59 {
            Err(DateTimeError::InvalidMinute)
        } else {
            Ok(Time {
                hour,
                minute,
            })
        }
    }

    pub fn parse(time: &str) -> Result<Time, DateTimeError> {
        let time = time.to_string();
        let time = time.split(":").collect::<Vec<&str>>();
        if time.len() != 2 {
            return Err(DateTimeError::UnspecifiedTime);
        }

        let hour = time[0].parse::<u8>().expect("Invalid hour.");
        let minute = time[1].parse::<u8>().expect("Invalid minute.");

        Time::new(hour, minute)
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