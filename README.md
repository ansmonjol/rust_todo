# rust_todo

A simple To-Do application built with Rust.

Commands:

- `add [text]`  
  Add a new todo item. If `[text]` is omitted, you will be prompted to enter the todo.  
  Examples:

  - `add`
  - `add buy groceries and milk`

- `list` or `ll`  
  View all todos.

- `remove [index]`  
  Remove a todo by its index. If `[index]` is omitted, you will see the current list and be prompted for the index.  
  Examples:

  - `remove`
  - `remove 0`

- `clear_all`  
  Clear all todos (confirmation required).

- `exit`  
  Exit the application.

- `help` or `-h`  
  Show the help message.
