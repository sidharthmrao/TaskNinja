use std::env;

mod tasks;
mod dates;
mod command_handler;
mod utils;
mod config;

use tasks::{TaskList};
use dates::{Date, Time};
use command_handler::{command_handler};
use crate::command_handler::CommandError;
use crate::config::Config;
use crate::utils::{save_tasks, SaveError};

fn display_command_response(response: Result<String, CommandError>, config: Config) {
    match response {
        Ok(message) => println!("{}", message.as_str()),
        Err(error) => println!("{}{}\x1b[0m{}", config.error_color, error, &config.default_color),
    }
}

fn display_save_response(response: Result<String, SaveError>, config: Config) {
    match response {
        Ok(message) => println!("{}{}", config.success_color, message.as_str()),
        Err(error) => println!("{}{}\x1b[0m{}", config.error_color, error, &config.default_color),
    }
}

fn main() {
    let mut config: Config;
    let mut read_conf = Config::read_from_file("config.json".to_string());
    match read_conf {
        Ok(conf) => {
            config = conf;
        },
        Err(e) => {
            config = Config::default();
            let _ = config.save_to_file();
            display_save_response(Err(e), config.clone());
        }
    }

    let args: Vec<String> = env::args().collect();
    let command = args[1..].to_owned();
    display_command_response(command_handler(command, config.clone()), config.clone());
}
