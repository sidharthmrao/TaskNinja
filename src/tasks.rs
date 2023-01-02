use crate::{DateTest, TimeTest};

pub struct Task {
    description: String,
    due_date: Option<DateTest>,
    due_time: Option<TimeTest>,
    complete: bool,
    flagged: bool,
}

impl Task {
    pub fn new() -> Task {
        Task {
            description: String::new(),
            due_date: None,
            due_time: None,
            complete: false,
            flagged: false
        }
    }

    pub fn from(description: String, due_date: Option<DateTest>, due_time: Option<TimeTest>, complete: bool, flagged: bool) -> Task {
        Task {
            description,
            due_date,
            due_time,
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

    pub fn edit_due_date(&mut self, due_date: Option<DateTest>) {
        self.due_date = due_date;
    }

    pub fn edit_due_time(&mut self, due_time: Option<TimeTest>) {
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
            Some(date) => {
                println!("Due Date: {}", date.as_calendar_date_string());
            },
            _ => { println!("Due Date: Not specified."); }
        }

        match &self.due_time {
            Some(time) => {
                println!("Due Time: {}", time.as_12_hour_time_string());
            },
            _ => { println!("Due Time: Not specified."); }
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