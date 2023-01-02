mod tasks;
mod dates;

use tasks::{TaskList, Task};
use dates::{DateTest, TimeTest};

fn main() {
    let mut task_list = TaskList::new();

    let task_1 = Task::from("Get into Cornell.".to_string(), DateTest::new(2022, "Sep", 12), TimeTest::new(12, 6), true, false);
    let task_2 = Task::from("Get into Cornell.".to_string(), DateTest::new(2022, "Sep", 12), TimeTest::new(12, 6), true, false);
    task_list.add_task(task_1);
    task_list.add_task(task_2);
    task_list.display();
}
