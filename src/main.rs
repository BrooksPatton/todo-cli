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

    fn mark(&mut self) {
        self.completed = 'x';
    }

    fn unmark(&mut self) {
        self.completed = ' ';
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

        println!("Displaying all tasks");

        for todo_item in &self.list {
            println!("{} - [{}] {}", count, todo_item.completed, todo_item.task);
            count += 1;
        }
    }

    fn seed_list(&mut self) {
        let task_1 = String::from("Add a task");
        let task_2 = String::from("Display all tasks");
        let task_3 = String::from("Mark tasks as completed");
        let task_4 = String::from("Remove a task");
        let task_5 = String::from("Unmark a task");

        self.add_task(task_1);
        self.add_task(task_2);
        self.add_task(task_3);
        self.add_task(task_4);
        self.add_task(task_5);

        self.mark_completed(4);
    }

    fn mark_completed(&mut self, id: usize) {
        self.list[id].mark();
    }

    fn unmark_completed(&mut self, id: usize) {
        self.list[id].unmark();
    }

    fn remove(&mut self, id: usize) {
        self.list.remove(id);
    }
}

enum Actions {
    Add(String),
    Mark(usize),
    UnMark(usize),
    Remove(usize),
    List
}

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let command = &arguments[1];
    let action = match command.as_str() {
        "add" => Actions::Add(arguments[2].clone()),
        "mark" => Actions::Mark(arguments[2].parse().unwrap()),
        "unmark" => Actions::UnMark(arguments[2].parse().unwrap()),
        "remove" => Actions::Remove(arguments[2].parse().unwrap()),
        _ => Actions::List
    };

    let mut list = TodoList::new();
    list.seed_list();

    match action {
        Actions::List => list.display_tasks(),
        Actions::Add(task) => {
            list.add_task(task);
            list.display_tasks();
        },
        Actions::Mark(id) => {
            list.mark_completed(id);
            list.display_tasks();
        },
        Actions::UnMark(id) => {
            list.unmark_completed(id);
            list.display_tasks();
        },
        Actions::Remove(id) => {
            list.remove(id);
            list.display_tasks();
        }
    };
}
