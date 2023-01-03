use crate::{save_tasks, TaskList};
use std::fmt;
use std::error::Error as StdError;
use indoc::{indoc};
use crate::Date;
use crate::dates::DateTimeError;
use crate::Time;
use crate::utils::read_tasks;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommandError {
    InvalidMainOperation(String),
    InvalidHelpOperation(String),
    MissingRequiredArgument(String, String),
    InvalidArgument(String, String),
    TaskNotFound(String),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandError::InvalidMainOperation(operation) => write!(f, "Invalid operation. '{}' not found. Run 'taskninja help' for a list of commands.", operation),
            CommandError::InvalidHelpOperation(operation) => write!(f, "Invalid help operation. Argument '{}' not found. Run 'taskninja help' for more info.", operation),
            CommandError::MissingRequiredArgument(operation, argument) => write!(f, "Missing required argument '{}' for operation '{}'. Run 'taskninja help {operation}' for more info.", argument, operation),
            CommandError::InvalidArgument(operation, argument) => write!(f, "Invalid argument '{}' for operation '{}'. Run 'taskninja help {operation}' for more info.", argument, operation),
            CommandError::TaskNotFound(task) => write!(f, "Task '{}' not found.", task),
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
            CommandError::TaskNotFound(_) => "Task not found.",
        }
    }
}

pub struct Response;

impl Response {
    fn help(help_token: &str) -> Result<String, CommandError> {
        match help_token.to_lowercase().as_str() {
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
                        help, -h, --help          Display detailed help about the add operation.
                        -t, --title         Title of the task. (Required)

                        -d, --description   Description of the task. (Optional)
                                             - Can be set without the -d or --description flag.
                        due, -D, --date          Due date of the task. (YYYY-MM-DD or YYYY-Month_name-DD) (Optional)
                        at, -T, --time          Due time of the task. (HH:MM) (Optional)
                        flag, -f, --flag          Mark the task as important. (Optional)
                        -p, --priority      Set priority of the task. (1 or higher) (Optional)

                    Examples:
                        taskninja add -t 'Get into Cornell.' -D 2022-09-12 -T 12:06 -r -f
                        taskninja add -t 'Ask out for Last Hurrah.' -D 2022-September-12
                        taskninja add -t 'Blah blah blah.' -d 'More nonsense' -f -p 2
                        taskninja add 'Go on a run' due 2022-09-12 at 12:06 flag
                "}.to_string()
            ),
            "delete" => Ok(
                indoc! {"
                    taskninja delete: Delete a task.
                    Usage: taskninja delete [ID]

                    Arguments for 'delete':
                        -h, --help          Display detailed help about the delete operation.

                    Examples:
                        taskninja delete 1
                "}.to_string()
            ),
            "complete" => Ok(
                indoc! {"
                    taskninja complete: Mark a task as complete.
                    Usage: taskninja complete [ID]

                    Arguments for 'complete':
                        -h, --help          Display detailed help about the complete operation.

                    Examples:
                        taskninja complete 1
                "}.to_string()
            ),
            "incomplete" => Ok(
                indoc! {"
                    taskninja incomplete: Mark a task as incomplete.
                    Usage: taskninja incomplete [ID]

                    Arguments for 'incomplete':
                        -h, --help          Display detailed help about the incomplete operation.

                    Examples:
                        taskninja incomplete 1
                "}.to_string()
            ),
            "list" => Ok(
                indoc! {"
                    taskninja list: List all tasks.
                    Usage: taskninja list [options]

                    Arguments for 'list':
                        -h, --help          Display detailed help about the list operation.
                        -c, --complete      List only complete tasks. (Optional)
                        -i, --incomplete    List only incomplete tasks. (Optional)
                        -f, --flagged       List only flagged tasks. (Optional)
                        -u, --unflagged     List only unflagged tasks. (Optional)
                        -t, --today          List only tasks due today. (Optional)

                    Examples:
                        taskninja list
                        taskninja list -c
                        taskninja list --today
                "}.to_string()
            ),

            _ => Err(CommandError::InvalidHelpOperation(help_token.to_string())),
        }
    }
}

pub(crate) fn command_handler(command: Vec<String>) -> Result<String, CommandError> {
    let mut task_list: TaskList;

    match read_tasks() {
        Ok(mut tasks) => {
            task_list = tasks;
        }
        Err(_) => {
            task_list = TaskList::new();
        }
    }

    if command.len() == 0 {
        return Response::help("help");
    }

    let response = match command[0].as_str() {
        "help" | "h" => {
            match command.len() {
                1 => Response::help("help"),
                _ => Response::help(&command[1]),
            }
        }
        "add" | "a" => {
            if command.len() == 1 {
                Err(CommandError::MissingRequiredArgument("Add".to_string(), "Title".to_string()))
            } else {
                let mut title: Option<String> = None;
                let mut description: Option<String> = None;
                let mut date: Option<Result<Date, DateTimeError>> = None;
                let mut time: Option<Result<Time, DateTimeError>> = None;
                let mut flag: bool = false;
                let mut priority: Option<u8> = None;

                let mut i = 1;
                while i < command.len() {
                    match command[i].as_str() {
                        "help" | "-h" | "--help" => return Response::help("add"),
                        "-t" | "--title" => {
                            if i + 1 < command.len() {
                                title = Some(command[i + 1].to_string());
                                i += 1;
                            } else {
                                return Err(CommandError::MissingRequiredArgument("Add".to_string(), "Title".to_string()));
                            }
                        }
                        "-d" | "--description" => {
                            if i + 1 < command.len() {
                                description = Some(command[i + 1].to_string());
                                i += 1;
                            } else {
                                return Err(CommandError::InvalidArgument("Add".to_string(), "Description".to_string()));
                            }
                        }
                        "due" | "-D" | "--date" => {
                            if i + 1 < command.len() {
                                date = Some(Date::parse(&command[i + 1]));
                                i += 1;
                            } else {
                                return Err(CommandError::InvalidArgument("Add".to_string(), "Date".to_string()));
                            }
                        }
                        "at" | "-T" | "--time" => {
                            if i + 1 < command.len() {
                                time = Some(Time::parse(&command[i + 1]));
                                i += 1;
                            } else {
                                return Err(CommandError::InvalidArgument("Add".to_string(), "Time".to_string()));
                            }
                        }
                        "flag" | "-f" | "--flag" => {
                            flag = true;
                        }
                        "-p" | "--priority" => {
                            if i + 1 < command.len() {
                                match command[i + 1].parse::<u8>() {
                                    Ok(p) => priority = Some(p),
                                    Err(_) => return Err(CommandError::InvalidArgument("Add".to_string(), "Priority".to_string())),
                                }
                                i += 1;
                            } else {
                                return Err(CommandError::InvalidArgument("Add".to_string(), "Priority".to_string()));
                            }
                        }
                        _ => {
                            if title.is_none() && i == 1 {
                                title = Some(command[i].to_string());
                            } else {
                                return Err(CommandError::InvalidArgument("Add".to_string(), command[i].to_string()));
                            }
                        }
                    }
                    i += 1;
                }

                if title.is_none() {
                    return Err(CommandError::MissingRequiredArgument("Add".to_string(), "Title".to_string()));
                } else {
                    task_list.new_task(title.as_ref().unwrap().to_string(), description, date, time, priority, false, flag);
                    let _ = save_tasks(task_list);
                    Ok(("'".to_owned() + &title.unwrap() + "' added.").to_string())
                }
            }

        },
        "delete" | "d" => {
            if command.len() == 1 {
                Err(CommandError::MissingRequiredArgument("Delete".to_string(), "ID".to_string()))
            } else {
                return if command[1] == "help" || command[1] == "-h" || command[1] == "--help" {
                    Response::help("delete")
                } else if command[1] == "all" || command[1] ==  "-a" || command[1] == "--all" {
                    task_list.tasks = Vec::new();
                    let _ = save_tasks(task_list);
                    Ok("All tasks deleted.".to_string())
                } else {
                    match command[1].parse::<usize>() {
                        Ok(id) => {
                            match task_list.remove_task(id - 1) {
                                Ok(ok) => Ok(ok),
                                Err(_) => Err(CommandError::TaskNotFound(command[1].to_string())),
                            }
                        }
                        Err(_) => Err(CommandError::InvalidArgument("Delete".to_string(), command[1].to_string())),
                    }
                }
            }
        },
        "complete" | "c" => {
            if command.len() == 1 {
                Err(CommandError::MissingRequiredArgument("Complete".to_string(), "ID".to_string()))
            } else {
                return if command[1] == "help" || command[1] == "-h" || command[1] == "--help" {
                    Response::help("complete")
                } else if command[1] == "all" || command[1] ==  "-a" || command[1] == "--all" {
                    for task in &mut task_list.tasks {
                        task.complete = true;
                    }
                    let _ = save_tasks(task_list);
                    Ok("All tasks completed.".to_string())
                } else {
                    match command[1].parse::<usize>() {
                        Ok(id) => {
                            match task_list.mark_task_complete(id - 1) {
                                Ok(ok) => Ok(ok),
                                Err(_) => Err(CommandError::TaskNotFound(command[1].to_string())),
                            }
                        }
                        Err(_) => Err(CommandError::InvalidArgument("Complete".to_string(), command[1].to_string())),
                    }
                }
            }
        },
        "incomplete" | "i" => {
            if command.len() == 1 {
                Err(CommandError::MissingRequiredArgument("Incomplete".to_string(), "ID".to_string()))
            } else {
                return if command[1] == "help" || command[1] == "-h" || command[1] == "--help" {
                    Response::help("incomplete")
                } else if command[1] == "all" || command[1] ==  "-a" || command[1] == "--all" {
                    for task in &mut task_list.tasks {
                        task.complete = false;
                    }
                    let _ = save_tasks(task_list);
                    Ok("All tasks marked incomplete.".to_string())
                } else {
                    match command[1].parse::<usize>() {
                        Ok(id) => {
                            match task_list.mark_task_incomplete(id - 1) {
                                Ok(ok) => Ok(ok),
                                Err(_) => Err(CommandError::TaskNotFound(command[1].to_string())),
                            }
                        }
                        Err(_) => Err(CommandError::InvalidArgument("Incomplete".to_string(), command[1].to_string())),
                    }
                }
            }
        },
        "list" | "l" => {
            if command.len() == 1 {
                Ok(task_list.to_string())
            } else {
                let mut filters: Vec<&str> = Vec::new();
                for i in command[1..].iter() {

                    match i.as_str() {
                        "h" | "help" | "-h" | "--help" => { return Response::help("list"); },
                        "a" | "all" | "-a" | "--all" => { return Ok(task_list.to_string()); },
                        "-c" | "--complete" => { filters.push("complete"); },
                        "-i" | "--incomplete" => { filters.push("incomplete"); },
                        "-f" | "--flagged" => { filters.push("flagged"); },
                        "-u" | "--unflagged" => { filters.push("unflagged"); },
                        "-t" | "--today" => { filters.push("today"); },
                        _ => { return Err(CommandError::InvalidArgument("List".to_string(), i.to_string())); },
                    }
                }

                Ok(task_list.filter_tasks_to_string(filters))
            }
        },
        _ => Err(CommandError::InvalidMainOperation(command[0].to_string()))
    };

    response
}