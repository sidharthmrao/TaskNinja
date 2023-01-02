mod tasks;
mod dates;

use tasks::{TaskList, Task};
use dates::{DateTest, TimeTest};

fn main() {
    let mut task_list = TaskList::new();

    let task_1 = Task::from("Get into Cornell.".to_string(), Some(DateTest::new(2022, "Sep", 12)), Some(TimeTest::new(12, 6)), true, false);
    let task_2 = Task::from("Ask out for Last Hurrah.".to_string(), Some(DateTest::new(2022, "Meow", 12)), None, false, true);
    let task_3 = Task::from("Blah blah blah.".to_string(), None, Some(TimeTest::new(25, 61)), false, false);
    task_list.add_task(task_1);
    task_list.add_task(task_2);
    task_list.add_task(task_3);
    task_list.display();
}
