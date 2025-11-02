use colored::*;
use std::process;
mod actions;

enum Command {
    Add(Option<String>),
    List,
    Remove(Option<i8>),
    ClearAll,
    Exit,
    Help,
}

impl Command {
    fn from_str(input: &str) -> Command {
        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        let command = parts[0];
        let args = if parts.len() > 1 {
            Some(parts[1])
        } else {
            None
        };

        match command {
            "add" => Command::Add(args.map(|s| s.to_string())),
            "list" | "ll" => Command::List,
            "remove" => {
                let index = args.and_then(|s| s.trim().parse::<i8>().ok());
                Command::Remove(index)
            }
            "clear_all" => Command::ClearAll,
            "exit" => Command::Exit,
            "help" | "-h" => Command::Help,
            _ => Command::Help,
        }
    }
}

fn main() {
    show_help();

    loop {
        println!("\n{}", "➜ Enter a command:".yellow());

        let mut input: String = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: &str = input.trim();

        let command = Command::from_str(input);
        match command {
            Command::Add(todo_text) => {
                if let Some(todo) = todo_text {
                    // Direct add: "add do this"
                    actions::add_todo(&todo);
                } else {
                    // Prompt for input: "add"
                    println!("{}", "➜ Enter a new todo:".yellow());
                    let mut todo = String::new();
                    std::io::stdin()
                        .read_line(&mut todo)
                        .expect("Failed to read line");
                    actions::add_todo(todo.trim());
                }
            }
            Command::List => actions::list_todos(),
            Command::Remove(index) => {
                if let Some(i) = index {
                    // Direct remove: "remove 0"
                    actions::remove_todo(i);
                } else {
                    // Prompt for index: "remove"
                    actions::list_todos();
                    println!("{}", "➜ Enter the index of the todo to remove:".yellow());
                    let mut index_input = String::new();
                    std::io::stdin()
                        .read_line(&mut index_input)
                        .expect("Failed to read line");
                    match index_input.trim().parse::<i8>() {
                        Ok(i) => actions::remove_todo(i),
                        Err(_) => {
                            println!("{}", "✗ Invalid index. Please enter a number.".red().bold())
                        }
                    }
                }
            }
            Command::ClearAll => {
                println!(
                    "{}",
                    "➜ Are you sure you want to clear all todos? (y/n):".yellow()
                );
                let mut confirm = String::new();
                std::io::stdin()
                    .read_line(&mut confirm)
                    .expect("Failed to read line");
                if confirm.trim() == "y" {
                    actions::clear_all_todos();
                } else {
                    println!("{}", "✗ Cancelled.".red().bold());
                }
            }
            Command::Exit => process::exit(0),
            Command::Help => show_help(),
        }
    }
}

fn show_help() {
    println!("\n{}", "━".repeat(60).bright_black());
    println!("\n{}\n", "📚 Available Commands".bright_cyan().bold());

    println!(
        "  {} {}",
        "add [text]".bright_green().bold(),
        "      - Add a new todo (prompts if no text provided)".white()
    );
    println!(
        "                   {}: {} {} {}",
        "Examples".dimmed(),
        "'add'".bright_white(),
        "or".dimmed(),
        "'add buy groceries and milk'".bright_white()
    );

    println!(
        "\n  {} {}",
        "list, ll".bright_green().bold(),
        "            - View all todos".white()
    );

    println!(
        "\n  {} {}",
        "remove [index]".bright_green().bold(),
        "  - Remove a todo (shows list if no index provided)".white()
    );
    println!(
        "                   {}: {} {} {}",
        "Examples".dimmed(),
        "'remove'".bright_white(),
        "or".dimmed(),
        "'remove 0'".bright_white()
    );

    println!(
        "\n  {} {}",
        "clear_all".bright_green().bold(),
        "            - Clear all todos".white()
    );

    println!(
        "\n  {} {}",
        "exit".bright_green().bold(),
        "            - Exit the application".white()
    );

    println!(
        "\n  {} {}",
        "help, -h".bright_green().bold(),
        "        - Show this help message".white()
    );

    println!("\n{}\n", "━".repeat(60).bright_black());
}
