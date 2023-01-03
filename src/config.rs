use std::fs::File;
use std::io::{Read, Write};

pub struct Config {
    pub config_file: String,
    pub data_file: String,
    pub time_24_hour: bool,
    pub date_numerical: bool,

    pub error_color: String,
    pub flag_color: String,
    pub success_color: String,
    pub default_color: String,
}

impl Config {
    pub fn default() -> Config {
        Config {
            config_file: String::from("config.toml"),
            data_file: String::from("data.toml"),
            time_24_hour: true,
            date_numerical: false,

            error_color: String::from("\x1b[31m"),
            flag_color: String::from("\x1b[4m"),
            success_color: String::from("\x1b[32m"),
            default_color: String::from("\x1b[0m"),
        }
    }

    pub fn from_file(file: String) -> Config {
        let mut config = Config::default();
        let mut file = File::open(file).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let toml = contents.parse::<toml::Value>().unwrap();
        config.time_24_hour = toml["time_24_hour"].as_bool().unwrap();
        config.date_numerical = toml["date_numerical"].as_bool().unwrap();
        config
    }

    pub fn to_file(&self, file: String) {
        let mut file = File::create(file).unwrap();
        let mut contents = String::new();
        contents.push_str("[config]\n");
        contents.push_str(&format!("time_24_hour = {}\n", self.time_24_hour));
        contents.push_str(&format!("date_numerical = {}\n", self.date_numerical));
        file.write_all(contents.as_bytes()).unwrap();
    }
}