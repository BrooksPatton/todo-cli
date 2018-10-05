# Making a todo cli in Rust

* [x] Introduction to Rust tutorial
* [x] Who am I?
* [x] What is Rust
* [x] Install Rust
* [x] Create a new project
* [x] Hello world
* [x] VS Code extensions
* [x] Where to find documentation
* [-] Collect CLI arguments into Vector
  * [x] `let arguments: Vec<String> = env::args().collect();`
  * [x] What is happening?
    * [x] `env::args().collect();`
    * [x] Creating variables
    * [x] Vectors
    * [x] Strings
* [ ] Create a way to store user inputs
  * [ ] Start with list all tasks
* [ ] Create an empty list
* [ ] Seed the list with default data
* [ ] List all tasks
* [ ] Add a new task
* [ ] Mark a task as complete
* [ ] Unmark a task
* [ ] Remove a task
* [ ] Build for production
  * [ ] Linux
  * [ ] Windows
  * [ ] Mac

## Challenge for you

* Instead of taking user input from arguments, run a loop and ask the user for their command every iteration.
* Implement a command for changing the task description
* Implement a custom sort command