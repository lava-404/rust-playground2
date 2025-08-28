use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    println!(" Welcome to your Rusty TODO CLI ");
    println!("Type \"help\" to see the commands.\n");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        display_help();
        return;
    }

    match args[1].as_str() {
        "add" => add_task(&args),
        "delete" => delete_task(&args),
        "done" => done_task(&args),
        "list" => list_task(),
        "help" => display_help(),
        _ => display_help(),
    }
}

fn add_task(args: &[String]) {
    if args.len() < 3 {
        println!("Usage: todo add <task description>");
        return;
    }

    let task: String = args[2..].join(" "); 
    let path = Path::new("todo.txt");

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .expect("Unable to open todo.txt");

    writeln!(file, "[ ] {}", task).expect("Unable to write task");
    println!("Task added: {}", task);
}

fn delete_task(args: &[String]) {
    if args.len() < 3 {
        println!(" Usage: todo delete <task number>");
        return;
    }

    let task_num: usize = args[2].parse().unwrap_or(0);
    if task_num == 0 {
        println!("Task numbers start from 1.");
        return;
    }

    let path = Path::new("todo.txt");
    let file = File::open(&path).expect("Unable to open todo.txt");
    let reader = BufReader::new(file);

    let tasks: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    if task_num > tasks.len() {
        println!("Invalid task number");
        return;
    }

    let mut new_tasks = tasks.clone();
    new_tasks.remove(task_num - 1);

    let mut file = File::create(&path).expect("Unable to rewrite todo.txt");
    for task in new_tasks {
        writeln!(file, "{}", task).unwrap();
    }

    println!("🗑️ Task {} deleted", task_num);
}

fn done_task(args: &[String]) {
    if args.len() < 3 {
        println!("Usage: todo done <task number>");
        return;
    }

    let task_num: usize = args[2].parse().unwrap_or(0);
    if task_num == 0 {
        println!("Task numbers start from 1.");
        return;
    }

    let path = Path::new("todo.txt");
    let file = File::open(&path).expect("Unable to open todo.txt");
    let reader = BufReader::new(file);

    let mut tasks: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    if task_num > tasks.len() {
        println!("Invalid task number");
        return;
    }

    if tasks[task_num - 1].starts_with("[x]") {
        println!("Task {} is already done!", task_num);
        return;
    }

    tasks[task_num - 1] = tasks[task_num - 1].replacen("[ ]", "[x]", 1);

    let mut file = File::create(&path).expect("Unable to rewrite todo.txt");
    for task in tasks {
        writeln!(file, "{}", task).unwrap();
    }

    println!("Task {} marked as done!", task_num);
}

fn list_task() {
    let path = Path::new("todo.txt");
    if !path.exists() {
        println!(" No tasks yet. Add one with: todo add \"your task\"");
        return;
    }

    let file = File::open(&path).expect("Unable to open todo.txt");
    let reader = BufReader::new(file);

    println!("Your TODOs:");
    for (i, line) in reader.lines().enumerate() {
        println!("{}. {}", i + 1, line.unwrap());
    }
}

fn display_help() {
    println!(
        r#"
Available commands:
  add <task>       - Add a new task
  list             - List all tasks
  done <num>       - Mark a task as done
  delete <num>     - Delete a task
  help             - Show this help message
"#
    );
}
