use std::fs::File;
use serde::{Serialize, Deserialize};
use serde_json::to_writer_pretty;
use crate::SaveError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub data_file: String,
    pub time_24_hour: bool,
    pub date_numerical: bool,

    pub error_color: String,
    pub flag_color: String,
    pub success_color: String,
    pub default_color: String,
    pub complete_color: String,
    pub incomplete_color: String,
}

impl Config {
    pub fn default() -> Config {
        Config {
            data_file: String::from("data/tasks.json"),
            time_24_hour: false,
            date_numerical: false,

            error_color: String::from("\x1b[31m"),
            flag_color: String::from("\x1b[4m"),
            success_color: String::from("\x1b[32m"),
            default_color: String::from("\x1b[0m"),
            complete_color: String::from("\x1b[36m"),
            incomplete_color: String::from("\x1b[31m"),
        }
    }

    pub fn save_to_file(&self) -> Result<String, SaveError> {
        std::fs::create_dir("data").unwrap_or_default();
        let file = File::create("data/config.json").unwrap();
        let write = to_writer_pretty(file, &self);

        match write {
            Ok(_) => Ok("Config saved successfully.".to_string()),
            Err(error) => Err(SaveError::FailedSave(error.to_string())),
        }
    }


    pub fn read_from_file(path: String) -> Result<Config, SaveError> {
        let file = File::open(&path);

        match file {
            Ok(file) => {
                let config = serde_json::from_reader(file);
                match config {
                    Ok(config) => Ok(config),
                    Err(error) => Err(SaveError::FailedRead(error.to_string())),
                }
            }
            Err(_) => Err(SaveError::FailedRead("Error reading config file, using default config.".to_string())),
        }
    }
}