use std::env;

struct TodoItem {
    name: String,
    completed: bool
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let command = &arguments[1];
    let todo_item;

    if command == "add" {
        todo_item = TodoItem {
            name: arguments[2].clone(),
            completed: false,
        };

        println!("name: {}, completed: {}", todo_item.name, todo_item.completed);
    }
}

// javascript

// var todoItem = {
    // name: arguments[2]
    // completed: false
// }