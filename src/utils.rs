use serde_json::to_writer_pretty;
use std::fs::File;
use crate::{Config, TaskList};
use std::fmt;
use std::error::Error as StdError;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SaveError {
    FailedSave(String),
    FailedRead(String),
}

impl fmt::Display for SaveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SaveError::FailedSave(e) => write!(f, "Error saving tasks: {}", e),
            SaveError::FailedRead(e) => write!(f, "Error reading tasks: {}", e),
        }
    }
}

impl StdError for SaveError {
    fn description(&self) -> &str {
        match self {
            SaveError::FailedSave(_) => "Error saving tasks.",
            SaveError::FailedRead(_) => "Error reading tasks.",
        }
    }
}

pub(crate) fn read_tasks(config: Config) -> Result<TaskList, SaveError> {
    let file = File::open(config.data_file);
    match file {
        Ok(file) => {
            let task_list = serde_json::from_reader(file);
            match task_list {
                Ok(task_list) => Ok(task_list),
                Err(error) => Err(SaveError::FailedRead(error.to_string())),
            }
        }
        Err(error) => Err(SaveError::FailedRead(error.to_string())),
    }
}

pub(crate) fn save_tasks(tasks: TaskList, config: Config) -> Result<String, SaveError> {
    let file = File::create(&config.data_file);
    match file {
        Ok(file) => {
            let write = to_writer_pretty(file, &tasks);
            match write {
                Ok(_) => Ok("Tasks saved successfully.".to_string()),
                Err(error) => Err(SaveError::FailedSave(error.to_string())),
            }
        }
        Err(_) => {
            let _ = std::fs::create_dir("data");
            let file = File::create(&config.data_file);
            let _ = to_writer_pretty(file.unwrap(), &tasks);
            Err(SaveError::FailedSave("Could not find data file. Using default location.".to_string()))
        }
    }
}