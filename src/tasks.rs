use crate::{Config, Date, Time};
use crate::dates::DateTimeError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    num: u8,
    title: String,
    description: Option<String>,
    due_date: Result<Date, DateTimeError>,
    due_time: Result<Time, DateTimeError>,
    pub(crate) complete: bool,
    flagged: bool,
}

impl Task {

    pub fn from(title: String, description: Option<String>, due_date: Option<Result<Date, DateTimeError>>, due_time: Option<Result<Time, DateTimeError>>, complete: bool, flagged: bool) -> Task {
        Task {
            num: 0,
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

    pub fn to_string(&self, config: Config) -> String {
        let success_color = config.success_color;
        let error_color = config.error_color;
        let flag_color = config.flag_color;
        let default_color = config.default_color;
        let complete_color = config.complete_color;
        let incomplete_color = config.incomplete_color;

        let mut response = String::new();

        let mut color_setup = String::new();

        if self.complete {
            color_setup.push_str(&complete_color);
        } else {
            if self.flagged {
                color_setup.push_str(&flag_color);
            }
            color_setup.push_str(&incomplete_color);
        }

        response.push_str(&format!("{}\n", color_setup));

        match self.num {
            0 => { response.push_str(&format!("{}\n", self.title)); },
            _ => { response.push_str(&format!("{}: {}\n", self.num, self.title)); },

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

        response.push_str(&"\x1b[0m");
        response.push_str(&default_color);

        response
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskList {
    pub(crate) tasks: Vec<Task>,
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

    pub fn add_task(&mut self, mut task: Task, priority: Option<u8>) {
        let mut temp = task.clone();

        match priority {
            Some(priority) => {
                temp.num = priority;
                self.tasks.insert(priority as usize, temp);
            }
            None => {
                temp.num = self.tasks.len() as u8 + 1;
                self.tasks.push(temp);
            }
        }
    }

    pub fn new_task(&mut self, title: String, description: Option<String>, due_date: Option<Result<Date, DateTimeError>>, due_time: Option<Result<Time, DateTimeError>>, priority: Option<u8>, complete: bool, flagged: bool) {
        self.add_task(Task::from(title, description, due_date, due_time, complete, flagged), priority);
    }

    pub fn mark_task_complete(&mut self, index: usize) -> Result<String, String> {
        match self.tasks.get_mut(index) {
            Some(task) => {
                task.mark_complete();
                let result = Ok(format!("'{}' marked complete.", task.title));
                result
            }
            None => Err(format!("Task not found: {}", index))
        }
    }

    pub fn mark_task_incomplete(&mut self, index: usize) -> Result<String, String> {
        match self.tasks.get_mut(index) {
            Some(task) => {
                task.mark_incomplete();
                let result = Ok(format!("'{}' marked incomplete.", task.title));
                result
            }
            None => Err(format!("Task not found: {}", index))
        }
    }

    pub fn remove_task(&mut self, index: usize) -> Result<String, String> {
        if index < self.tasks.len() {
            let task = self.tasks.remove(index);
            Ok(format!("'{}' removed.", task.title))
        } else {
            Err(format!("Task not found: {}", index))
        }
    }

    pub fn to_string(&self, config: Config) -> String {
        let mut response = String::new();

        for task in self.tasks.iter() {
            response.push_str(&format!("{}\n", task.to_string(config.clone())));
        }

        response
    }

    pub fn display(&self, config: Config) {
        println!("{}", self.to_string(config));
    }

    pub fn list_tasks_complete(&self, tasks: Vec<Task>) -> Vec<Task> {
        let mut response = Vec::new();

        for task in tasks {
            if task.complete {
                response.push(task);
            }
        }

        response
    }

    pub fn list_tasks_incomplete(&self, tasks: Vec<Task>) -> Vec<Task> {
        let mut response = Vec::new();

        for task in tasks {
            if !task.complete {
                response.push(task);
            }
        }

        response
    }

    pub fn list_tasks_flagged(&self, tasks: Vec<Task>) -> Vec<Task> {
        let mut response = Vec::new();

        for task in tasks {
            if task.flagged {
                response.push(task);
            }
        }

        response
    }

    pub fn list_tasks_unflagged(&self, tasks: Vec<Task>) -> Vec<Task> {
        let mut response = Vec::new();

        for task in tasks {
            if !task.flagged {
                response.push(task);
            }
        }

        response
    }

    pub fn list_tasks_due_today(&self, tasks: Vec<Task>) -> Vec<Task> {
        let mut response = Vec::new();

        for task in tasks {
            match &task.due_date {
                Ok(date) => {
                    if date.is_today() {
                        response.push(task);
                    }
                }
                Err(_) => {}
            }
        }

        response
    }

    pub fn filter_tasks(&self, filters: Vec<&str>) -> Vec<Task> {
        let mut response = self.tasks.clone();

        for filter in filters.iter() {
            match filter.to_string().as_str() {
                "complete" => {
                    response = self.list_tasks_complete(response.clone());
                },
                "incomplete" => {
                    response = self.list_tasks_incomplete(response.clone());
                },
                "flagged" => {
                    response = self.list_tasks_flagged(response.clone());
                },
                "unflagged" => {
                    response = self.list_tasks_unflagged(response.clone());
                },
                "due_today" => {
                    response = self.list_tasks_due_today(response.clone());
                },
                _ => {}
            }
        }

        response
    }

    pub fn filter_tasks_to_string(&self, filters: Vec<&str>, config: Config) -> String {
        let mut response = String::new();

        for task in self.filter_tasks(filters).iter() {
            response.push_str(&format!("{}\n", task.to_string(config.clone())));
        }

        response
    }
}
