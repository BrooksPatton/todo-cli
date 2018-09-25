use std::env;

struct TodoItem {
    task: String,
    completed: char,
}

impl TodoItem {
    fn new(task: String) -> TodoItem {
        TodoItem {
            task: task,
            completed: ' '
        }
    }
}

struct TodoList {
    list: Vec<TodoItem>
}

impl TodoList {
    fn new() -> TodoList {
        let list = Vec::new();

        TodoList {
            list: list
        }
    }

    fn add_task(&mut self, task: String) {
        let todo_item = TodoItem::new(task);

        self.list.push(todo_item);
    }

    fn display_tasks(&self) {
        let mut count = 0;

        for todo_item in &self.list {
            println!("{} - [{}] {}", count, todo_item.completed, todo_item.task);
            count += 1;
        }
    }
}

enum Actions {
    Add,
    Done,
    Undone,
    Delete
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    // println!("{:?}", arguments);
    let command;

    if arguments[1] == "add" {
        let task = String::from(arguments[2]);
        command = Arguments::Add(arguments[2]);
    }

    let mut list = TodoList::new();
    let first_task = String::from("Mop floor");
    let second_task = String::from("Grocery shopping");

    list.add_task(first_task);
    list.add_task(second_task);
    list.display_tasks();
}
