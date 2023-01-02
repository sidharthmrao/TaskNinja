use crate::{DateTest, TimeTest};
use crate::dates::DateTimeError;

pub struct Task {
    description: String,
    due_date: Result<DateTest, DateTimeError>,
    due_time: Result<TimeTest, DateTimeError>,
    complete: bool,
    flagged: bool,
}

impl Task {
    pub fn new() -> Task {
        Task {
            description: String::new(),
            due_date: Err(DateTimeError::UnspecifiedDate),
            due_time: Err(DateTimeError::UnspecifiedTime),
            complete: false,
            flagged: false
        }
    }

    pub fn from(description: String, due_date: Option<Result<DateTest, DateTimeError>>, due_time: Option<Result<TimeTest, DateTimeError>>, complete: bool, flagged: bool) -> Task {
        Task {
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
            flagged
        }
    }

    pub fn mark_complete(&mut self) {
        self.complete = true;
    }

    pub fn mark_incomplete(&mut self) {
        self.complete = false;
    }

    pub fn edit_due_date(&mut self, due_date: Result<DateTest, DateTimeError>) {
        self.due_date = due_date;
    }

    pub fn edit_due_time(&mut self, due_time: Result<TimeTest, DateTimeError>) {
        self.due_time = due_time;
    }

    pub fn edit_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn display(&self, number: Option<i32>) {
        let mut color_setup = String::new();

        if self.complete {
            color_setup.push_str("\x1b[32m");
        } else {
            if self.flagged {
                color_setup.push_str("\x1b[4m");
            }
            color_setup.push_str("\x1b[31m");
        }

        println!("{}", color_setup);

        match number {
            Some(number) => println!("{}: {}", number, self.description),
            _ => println!("{}", self.description),
        }

        match &self.due_date {
            Ok(date) => {
                println!("Due Date: {}", date.as_calendar_date_string());
            },
            Err(E) => match E {
                    DateTimeError::UnspecifiedDate => { println!("Date Time: Not specified."); },
                    _ => { println!("Date Time: Invalid."); }
                }
        }

        match &self.due_time {
            Ok(time) => {
                println!("Due Time: {}", time.as_12_hour_time_string());
            },
            Err(E) => {
                match E {
                    DateTimeError::UnspecifiedTime => { println!("Due Time: Not specified."); },
                    _ => { println!("Due Time: Invalid."); }
                }
            }
        }

        println!("Complete: {}", self.complete);
    }
}

pub struct TaskList {
    tasks: Vec<Task>,
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

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn display(&self) {
        for (i, task) in self.tasks.iter().enumerate() {
            task.display(Some(i as i32 + 1));
        }
    }
}