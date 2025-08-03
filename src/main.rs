mod reminder;
mod data_manager;

use std::env::args;
use reminder::{Reminder, CommandType};
use data_manager::{save_reminder_to_file, get_remdiners, remove_reminder};
use rand::Rng;


fn main() {
    let arguments: Vec<String> = args().collect();

    if !check_args(&arguments) {
        return;
    }

    let command_str = get_command(&arguments[1]);

    if let Some(command) = command_str {
        handle_command(command, &arguments);
    } else {
        eprintln!("Invalid command: {}", arguments[1]);
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

fn handle_command(command: CommandType, args: &[String]) {
    match command {
        CommandType::Add => {
            println!("Adding a new reminder...");
            let id = rand::thread_rng().gen_range(1..100000);
            let reminder = Reminder::new(id, args[2].clone(), args[3].clone(), args[4].clone());
            save_reminder_to_file(&reminder);
        }
        CommandType::List => {
            println!("\n#################################");
            println!("Listing all reminders:");
            println!("#################################\n");
            
            let reminders = data_manager::get_remdiners();
            
            if reminders.is_empty() {
                println!("No reminders found.");
            }
            
            println!("----------------------------------------------------------------------");
            for r in reminders {
                println!("ID: {}, Title: {}, Description: {}, Due Date: {}", r.id, r.title, r.description, r.due_date);
            println!("----------------------------------------------------------------------");
            }
            
            println!("\n")
        }
        CommandType::Remove => {
            println!("Removing a reminder...");
            data_manager::remove_reminder(args[2].parse().unwrap()).expect("Failed to remove reminder");
        }
        CommandType::Update => println!("Updating a reminder..."),
    }
}

fn check_args(args: &[String]) -> bool {
    let command = &args[1].to_lowercase();

    match args.len() {
        2 => {
            if command == "list" {
                true
            } else {
                println!("Usage: cli_reminder_tool list");
                false
            }
        }
        3 => {
            if command == "remove" || command == "update" {
                true
            } else {
                println!("Usage: cli_reminder_tool remove <id> | update <id> <title> <description> <due_date>");
                false
            }
        }
        _ => {
            if args.len() < 5 {
                println!("Usage: cli_reminder_tool add <title> <description> <due_date>");
                return false;
            }

            true
        }
    }
}

