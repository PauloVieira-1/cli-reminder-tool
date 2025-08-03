mod reminder;
mod data_manager;

use std::env::args;
use reminder::{Reminder, CommandType};
use data_manager::save_reminder_to_file;
use rand::Rng;

fn main() {
    let args: Vec<String> = args().collect();

    if !check_args(&args) {
        return;
    }

    let command_str = get_command(&args[1]);
    let id = rand::thread_rng().gen_range(1..100000);
    let reminder = Reminder::new(id, args[2].clone(), args[3].clone(), args[4].clone());
   
    if let Some(command) = command_str {
        handle_command(command, reminder);
    } else {
        eprintln!("Invalid command: {}", args[1]);
    }
}

fn get_command(command_str: &str) -> Option<CommandType> {
    match command_str.to_lowercase().as_str() {
        "add" => Some(CommandType::Add),
        "list" => Some(CommandType::List),
        "remove" => Some(CommandType::Remove),
        "update" => Some(CommandType::Update),
        _ => None,
    }
}

fn handle_command(command: CommandType, reminder: Reminder) {
    match command {
        CommandType::Add => {
            println!("Adding a new reminder...");
            save_reminder_to_file(&reminder);
        }
        CommandType::List => println!("Listing all reminders..."),
        CommandType::Remove => println!("Removing a reminder..."),
        CommandType::Update => println!("Updating a reminder..."),
    }
}

fn check_args(args: &[String]) -> bool {
    if args.len() < 5 {
        eprintln!("Usage: <command> <title> <description> <due_date>");
        return false;
    }
    true
}
