use crate::{save_tasks, TaskList};
use std::fmt;
use std::error::Error as StdError;
use indoc::{indoc};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommandError {
    InvalidMainOperation(String),
    InvalidHelpOperation(String),
    MissingRequiredArgument(String, String),
    InvalidArgument(String, String),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandError::InvalidMainOperation(operation) => write!(f, "Invalid main operation. '{}' not found.", operation),
            CommandError::InvalidHelpOperation(operation) => write!(f, "Invalid help operation. '{}' not found.", operation),
            CommandError::MissingRequiredArgument(operation, argument) => write!(f, "Missing required argument '{}' for operation '{}'.", argument, operation),
            CommandError::InvalidArgument(operation, argument) => write!(f, "Invalid argument '{}' for operation '{}'.", argument, operation),
        }
    }
}

impl StdError for CommandError {
    fn description(&self) -> &str {
        match self {
            CommandError::InvalidMainOperation(_) => "Invalid main operation.",
            CommandError::InvalidHelpOperation(_) => "Invalid help operation.",
            CommandError::MissingRequiredArgument(_, _) => "Missing required argument.",
            CommandError::InvalidArgument(_, _) => "Invalid argument.",
        }
    }
}

pub struct Response;

impl Response {
    fn help(help_token: &str) -> Result<String, CommandError> {
        match help_token {
            "help" => Ok(
                indoc! {"
                    TaskNinja: A command line task manager.
                    Usage: taskninja [operation] [arguments]

                    Operations:
                        help, h         Display this help message or get more detailed help on an operation.
                                         - e.g. 'taskninja help' or 'taskninja help add'
                        add, a          Add a new task.
                        delete, d       Delete a task.
                        complete, c     Mark a task as complete.
                        incomplete, i   Mark a task as incomplete.
                        list, l         List all tasks.
                        search, s       Search for tasks.
                        edit, e         Edit a task.
                "}.to_string()
            ),
            "add" => Ok(
                indoc! {"
                    taskninja add: Add a new task.
                    Usage: taskninja add [title] [options]

                    Arguments for 'add':
                        -h, --help          Display detailed help about the add operation.
                        -t, --title         Title of the task. (Required)

                        -d, --description   Description of the task. (Optional)
                                             - Can be set without the -d or --description flag.
                        -D, --date          Due date of the task. (YYYY-MM-DD or YYYY-Month_name-DD) (Optional)
                        -T, --time          Due time of the task. (HH:MM) (Optional)
                        -f, --flag          Mark the task as important. (Optional)
                        -p, --priority      Set priority of the task. (1 or higher) (Optional)

                    Examples:
                        taskninja add -t 'Get into Cornell.' -D 2022-09-12 -T 12:06 -r -f
                        taskninja add -t 'Ask out for Last Hurrah.' -D 2022-September-12
                        taskninja add -t 'Blah blah blah.' -d 'More nonsense' -f -p 2
                "}.to_string()
            ),
            _ => Err(CommandError::InvalidHelpOperation(help_token.to_string())),
        }
    }
}

pub(crate) fn command_handler(command: Vec<String>, task_list: &mut TaskList) -> Result<String, CommandError> {
    if command.len() == 0 {
        return Response::help("help");
    }

    let response = match command[0].as_str() {
        "help" => {
            match command.len() {
                1 => Response::help("help"),
                _ => Response::help(&command[1]),
            }
        },
        "add" => {
            if command.len() == 1 {
                Err(CommandError::MissingRequiredArgument("Add".to_string(), "Title".to_string()))
            } else {
                match command[1].as_str() {
                    "-h" | "--help" => Response::help("add"),
                    "-t" | "--title" => Err(CommandError::MissingRequiredArgument("Add".to_string(), "Title".to_string())),
                    "-d" | "--description" => Err(CommandError::InvalidArgument("Add".to_string(), "Description".to_string())),
                    "-D" | "--date" => Err(CommandError::InvalidArgument("Add".to_string(), "Date".to_string())),
                    "-T" | "--time" => Err(CommandError::InvalidArgument("Add".to_string(), "Time".to_string())),
                    "-f" | "--flag" => Err(CommandError::MissingRequiredArgument("Add".to_string(), "Title".to_string())),
                    "-p" | "--priority" => Err(CommandError::InvalidArgument("Add".to_string(), "Priority".to_string())),

                    _ => {
                        task_list.new_task(command[1].to_string(), None, None, None, None, false, false);
                        Ok(("'".to_owned() + &command[1] + "' added.").to_string())
                    }
                }
            }
        }
        _ => {
            println!("{} not found. Type 'help' for a list of commands.", command[0]);
            Err(CommandError::InvalidHelpOperation(command[0].to_string()))
        }
    };

    let _ = save_tasks(&task_list);

    response
}