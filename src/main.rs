use std::env;

mod tasks;
mod dates;
mod command_handler;
mod utils;

use tasks::{TaskList};
use dates::{Date, Time};
use command_handler::{command_handler};
use crate::command_handler::CommandError;
use crate::utils::{save_tasks, SaveError};

fn display_command_response(response: Result<String, CommandError>) {
    match response {
        Ok(message) => println!("\x1b[32m{}", message.as_str()),
        Err(error) => println!("\x1b[31m{}\x1b[0m", error),
    }
}

fn display_save_response(response: Result<String, SaveError>) {
    match response {
        Ok(message) => println!("\x1b[32m{}", message.as_str()),
        Err(error) => println!("\x1b[31m{}\x1b[0m", error),
    }
}

fn main() {
    let mut task_list = TaskList::new();

    let args: Vec<String> = env::args().collect();
    let command = args[1..].to_owned();
    display_command_response(command_handler(command, &mut task_list));
}
