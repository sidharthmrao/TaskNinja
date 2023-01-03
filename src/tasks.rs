use crate::{Date, Time};
use crate::dates::DateTimeError;
use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;

pub struct Task {
    title: String,
    description: Option<String>,
    due_date: Result<Date, DateTimeError>,
    due_time: Result<Time, DateTimeError>,
    complete: bool,
    flagged: bool,
}

impl Serialize for Task {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Task", 6)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("description", &self.description)?;
        state.serialize_field("due_date", &self.due_date)?;
        state.serialize_field("due_time", &self.due_time)?;
        state.serialize_field("complete", &self.complete)?;
        state.serialize_field("flagged", &self.flagged)?;
        state.end()
    }
}

impl Task {

    pub fn from(title: String, description: Option<String>, due_date: Option<Result<Date, DateTimeError>>, due_time: Option<Result<Time, DateTimeError>>, complete: bool, flagged: bool) -> Task {
        Task {
            title,
            description,
            due_date: match due_date {
                Some(date) => date,
                None => Err(DateTimeError::UnspecifiedDate),
            },
            due_time: match due_time {
                Some(time) => time,
                None => Err(DateTimeError::UnspecifiedTime),
            },
            complete,
            flagged,
        }
    }

    pub fn mark_complete(&mut self) {
        self.complete = true;
    }

    pub fn mark_incomplete(&mut self) {
        self.complete = false;
    }

    pub fn edit_due_date(&mut self, due_date: Result<Date, DateTimeError>) {
        self.due_date = due_date;
    }

    pub fn edit_due_time(&mut self, due_time: Result<Time, DateTimeError>) {
        self.due_time = due_time;
    }

    pub fn edit_title(&mut self, title: String) { self.title = title; }

    pub fn edit_description(&mut self, description: Option<String>) { self.description = description; }

    pub fn to_string(&self, number: Option<i32>) -> String {
        let mut response = String::new();

        let mut color_setup = String::new();

        if self.complete {
            color_setup.push_str("\x1b[32m");
        } else {
            if self.flagged {
                color_setup.push_str("\x1b[4m");
            }
            color_setup.push_str("\x1b[31m");
        }

        response.push_str(&format!("{}\n", color_setup));

        match number {
            Some(number) => { response.push_str(&format!("{}: {}\n", number, self.title)); },
            _ => { response.push_str(&format!("{}\n", self.title)); },
        }

        match &self.description {
            Some(description) => { response.push_str(&format!("Description: {}\n", description)); },
            _ => { response.push_str(&format!("Description: Not specified.\n")); },
        }

        match &self.due_date {
            Ok(date) => { response.push_str(&format!("Due Date: {}\n", date.as_calendar_date_string())); }
            Err(e) => match e {
                DateTimeError::UnspecifiedDate => { response.push_str(&format!("Due Date: Not specified.\n")); }
                _ => { response.push_str(&format!("Due Date: Invalid. {}\n", e)); }
            }
        }

        match &self.due_time {
            Ok(time) => { response.push_str(&format!("Due Time: {}\n", time.as_12_hour_time_string())); }
            Err(e) => {
                match e {
                    DateTimeError::UnspecifiedTime => { response.push_str(&format!("Due Time: Not specified.\n")); }
                    _ => { response.push_str(&format!("Due Time: Invalid. {e}\n")); }
                }
            }
        }

        response.push_str(&format!("Complete: {}", self.complete));

        response.push_str(&format!("\x1b[0m"));

        response
    }
}

pub struct TaskList {
    tasks: Vec<Task>,
}

impl Serialize for TaskList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("TaskList", 1)?;
        state.serialize_field("tasks", &self.tasks)?;
        state.end()
    }
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList {
            tasks: Vec::new()
        }
    }

    pub fn from(tasks: Vec<Task>) -> TaskList {
        TaskList {
            tasks
        }
    }

    pub fn add_task(&mut self, task: Task, priority: Option<u8>) {
        match priority {
            Some(priority) => {
                self.tasks.insert(priority as usize, task);
            }
            None => {
                self.tasks.push(task);
            }
        }
    }

    pub fn new_task(&mut self, title: String, description: Option<String>, due_date: Option<Result<Date, DateTimeError>>, due_time: Option<Result<Time, DateTimeError>>, priority: Option<u8>, complete: bool, flagged: bool) {
        self.add_task(Task::from(title, description, due_date, due_time, complete, flagged), priority);
    }

    pub fn display(&self) {
        for (i, task) in self.tasks.iter().enumerate() {
            println!("{}", task.to_string(Some(i as i32 + 1)));
        }
    }
}
