use colored::*;
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

pub fn add_todo(todo: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("todos.txt")
        .expect("Failed to open file");

    file.write_all(format!("{}\n", todo).as_bytes())
        .expect("Failed to write to file");

    println!(
        "{} {}\n",
        "✓ Todo added:".green().bold(),
        todo.bright_white()
    );
}

pub fn list_todos() {
    // read the todos from the file
    let mut file = File::open("todos.txt").expect("Failed to open file");
    let mut todos: Vec<u8> = Vec::new();
    file.read_to_end(&mut todos).expect("Failed to read file");

    let todos_str = String::from_utf8(todos).expect("Failed to convert to string");
    let lines: Vec<&str> = todos_str.lines().filter(|line| !line.is_empty()).collect();

    if lines.is_empty() {
        println!("{}\n", "ℹ No todos yet!".yellow());
        return;
    }

    println!("{}", "📋 Your Todos:".bright_blue().bold());
    for (index, todo) in lines.iter().enumerate() {
        println!(
            "  {} {}",
            format!("[{}]", index).bright_magenta().bold(),
            todo.cyan()
        );
    }
    println!();
}

pub fn remove_todo(index: i8) {
    // read the todos from the file
    let mut file = File::open("todos.txt").expect("Failed to open file");
    let mut todos: Vec<u8> = Vec::new();
    file.read_to_end(&mut todos).expect("Failed to read file");

    let todos_str = String::from_utf8(todos).expect("Failed to convert to string");
    let lines: Vec<&str> = todos_str.lines().filter(|line| !line.is_empty()).collect();

    if lines.is_empty() {
        println!("{}\n", "⚠ No todos to remove!".yellow());
        return;
    }

    let index: usize = match index.try_into() {
        Ok(i) => i,
        Err(_) => {
            println!("{}", "✗ Invalid index.".red().bold());
            return;
        }
    };

    if index >= lines.len() {
        println!("{}", "✗ Index out of range.".red().bold());
        return;
    }

    let mut lines: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
    let removed_todo = lines.remove(index);

    let new_todos = lines.join("\n")
        + if !lines.is_empty() {
            "\n"
        } else {
            "No todos left.\n   "
        };

    // Write back to file (opening with write + truncate)
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("todos.txt")
        .expect("Failed to open file for writing");
    file.write_all(new_todos.as_bytes())
        .expect("Failed to write to file");

    println!(
        "{} {} {}\n",
        "✓ Todo removed:".green().bold(),
        format!("[{}]", index).bright_magenta().bold(),
        removed_todo.strikethrough().bright_black()
    );
}

pub fn clear_all_todos() {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("todos.txt")
        .expect("Failed to open file for writing");
    file.write_all(b"").expect("Failed to write to file");

    println!("{}", "✓ All todos cleared.".green().bold());
}
