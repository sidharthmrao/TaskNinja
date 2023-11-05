# TaskNinja
Rust CLI/TUI for task management.

##### Build from source
Install Rust and run `cargo build`

##### Usage
```
taskninja [operation] [arguments]

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

___

taskninja add: Add a new task.
Usage: taskninja add [title] [options]

Arguments for 'add':
    help, -h, --help    Display detailed help about the add operation.
    -t, --title         Title of the task. (Required)

    -d, --description   Description of the task. (Optional)
                         - Can be set without the -d or --description flag.
    due, -D, --date     Due date of the task. (YYYY-MM-DD or YYYY-Month_name-DD) (Optional)
    at, -T, --time      Due time of the task. (HH:MM) (Optional)
    flag, -f, --flag    Mark the task as important. (Optional)
    -p, --priority      Set priority of the task. (1 or higher) (Optional)

Examples:
    taskninja add -t 'Get into Cornell.' -D 2022-09-12 -T 12:06 -r -f
    taskninja add -t 'Ask out for Last Hurrah.' -D 2022-September-12
    taskninja add -t 'Blah blah blah.' -d 'More nonsense' -f -p 2
    taskninja add 'Go on a run' due 2022-09-12 at 12:06 flag

___

taskninja delete: Delete a task.
Usage: taskninja delete [ID]

Arguments for 'delete':
    -h, --help          Display detailed help about the delete operation.

Examples:
    taskninja delete 1

___

taskninja complete: Mark a task as complete.
Usage: taskninja complete [ID]

Arguments for 'complete':
    -h, --help          Display detailed help about the complete operation.

Examples:
    taskninja complete 1

___

taskninja incomplete: Mark a task as incomplete.
Usage: taskninja incomplete [ID]

Arguments for 'incomplete':
    -h, --help          Display detailed help about the incomplete operation.

Examples:
    taskninja incomplete 1

___

Usage: taskninja list [options]

Arguments for 'list':
    -h, --help          Display detailed help about the list operation.
    -c, --complete      List only complete tasks. (Optional)
    -i, --incomplete    List only incomplete tasks. (Optional)
    -f, --flagged       List only flagged tasks. (Optional)
    -u, --unflagged     List only unflagged tasks. (Optional)
    -t, --today         List only tasks due today. (Optional)

Examples:
    taskninja list
    taskninja list -c
    taskninja list --today

___

taskninja search: Search for tasks.
Usage: taskninja search [query] [options]

Arguments for 'search':
    -h, --help          Display detailed help about the search operation.
    -e, --exact         Search for an exact match. (Optional)

Examples:
    taskninja search 'shopping'
    taskninja search 'Go shopping.' -e
```
