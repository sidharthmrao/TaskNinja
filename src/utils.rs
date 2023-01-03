use serde_json::to_writer_pretty;
use serde::Serializer;
use std::fs::File;
use tasks::Task;
use crate::{TaskList, tasks};
use std::fmt;
use std::error::Error as StdError;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SaveError {
    SaveError(String),
    ReadError(String),
}

impl fmt::Display for SaveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SaveError::SaveError(error) => write!(f, "Error saving tasks: {}", error),
            SaveError::ReadError(error) => write!(f, "Error reading tasks: {}", error),
        }
    }
}

impl StdError for SaveError {
    fn description(&self) -> &str {
        match self {
            SaveError::SaveError(_) => "Error saving tasks.",
            SaveError::ReadError(_) => "Error reading tasks.",
        }
    }
}

// pub(crate) fn read_tasks() -> Result<TaskList, SaveError> {
//     let file = File::open("tasks.json");
//     match file {
//         Ok(file) => {
//             let task_list = serde_json::from_reader(file);
//             match task_list {
//                 Ok(task_list) => Ok(task_list),
//                 Err(error) => Err(SaveError::ReadError(error.to_string())),
//             }
//         }
//         Err(error) => Err(SaveError::ReadError(error.to_string())),
//     }
// }

pub(crate) fn save_tasks(tasks: &TaskList) -> Result<String, SaveError> {
    let mut file = File::create("tasks.json").unwrap();
    let write = to_writer_pretty(file, tasks);

    match write {
        Ok(_) => Ok("Tasks saved successfully".to_string()),
        Err(error) => Err(SaveError::SaveError(error.to_string())),
    }
}