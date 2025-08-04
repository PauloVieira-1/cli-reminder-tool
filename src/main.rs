mod reminder;
mod data_manager;
mod timer;

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
            let reminder = Reminder::new(id, args[2].clone(), args[3].clone(), args[4].clone(), args[5].clone());
            save_reminder_to_file(&reminder);
        }
        CommandType::List => {

            let longest_length = get_longest_vector_length(&data_manager::get_remdiners());

            println!("\n#################################");
            println!("Listing all reminders:");
            println!("#################################\n");
            
            let reminders = data_manager::get_remdiners();
            
            if reminders.is_empty() {
                println!("No reminders found.");
            }
            
            println!("{:-^1$}", "", longest_length);
            for r in reminders {
                println!("ID: {}, Title: {}, Description: {}, Due Date: {}, Time Stamp: {}", r.id, r.title, r.description, r.due_date, r.timestamp);
            println!("{:-^1$}", "", longest_length);
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

    println!("args: {:?}", args);
    let expected_argument_count = match command.as_str() {
        "list" => 2,
        "remove" | "update" => 3,
        "add" => 6,
        _ => {
            println!("Invalid command: {}", command);
            return false;
        }
    };

    if args.len() != expected_argument_count {
        println!(
            "Usage: cli_reminder_tool {} <{}>",
            command,
            match command.as_str() {
                "list" => "".to_string(),
                "remove" => "<id>".to_string(),
                "update" => "<id> <title> <description> <due_date:YYYY-MM-DD>".to_string(),
                "add" => "<title> <description> <due_date:YYYY-MM-DD> <time:HH:MM>".to_string(),
                _ => unreachable!(),
            }
        );
        return false;
    }

    if command == "add"{
        let date_time = format!("{} {}", args[4], args[5]);
        if !check_date_time(&date_time) {
            println!("Invalid date or time format. Please use 'YYYY-MM-DD HH:MM'.");
            return false;
        }
    }

    true
}

fn check_date_time (date_time: &str) -> bool {
    let parts: Vec<&str> = date_time.split_whitespace().collect();
    if parts.len() != 2 {
        return false;
    }

    let date_parts: Vec<&str> = parts[0].split('-').collect();
    if date_parts.len() != 3 {
        return false;
    }

    let time_parts: Vec<&str> = parts[1].split(':').collect();
    if time_parts.len() != 2 {
        return false;
    }
    true
}


fn get_longest_vector_length(vectors: &Vec<Reminder>) -> usize {

    let mut longest_length = 0;

    for reminder in vectors {
        let reminder_string = format!("ID: {}, Title: {}, Description: {}, Due Date: {}, Time Stamp: {}", reminder.id, reminder.title, reminder.description, reminder.due_date, reminder.timestamp);
        if reminder_string.len() > longest_length {
            longest_length = reminder_string.len(); 
        }
    }
    longest_length
}

