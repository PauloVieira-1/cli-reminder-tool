mod reminder;
mod data_manager;
mod timer;
mod watcher;
mod command_handler;

use std::env::args;
use reminder::{Reminder};
use data_manager::{save_reminder_to_file, get_remdiners, remove_reminder};
// use timer::start_timer;
use watcher::watch_reminders;
use command_handler::{add_command, list_reminders, remove_command, update_command, clear_command, CommandType};


#[tokio::main]
async fn main() {
    let arguments: Vec<String> = args().collect();

    if !check_args(&arguments) {
        return;
    }

    let command_str = get_command(&arguments[1]);

    if let Some(command) = command_str {
        handle_command(command, &arguments).await;
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
        "watch" => Some(CommandType::Watch),
        "clear" => Some(CommandType::Clear),
        _ => None,
    }
}

async fn handle_command(command: CommandType, args: &[String]) {
    match command {
        CommandType::Add => {
            add_command(args);
        }
        CommandType::List => list_reminders(),
        CommandType::Remove => remove_command(args),
        CommandType::Update => update_command(args),
        CommandType::Clear => clear_command(),
        CommandType::Watch => {
            println!("Starting reminder watcher...");
            watch_reminders().await.expect("Failed to start watcher");
        }
    }
}

fn check_args(args: &[String]) -> bool {
    let command = &args[1].to_lowercase();

    println!("args: {:?}", args);
    let expected_argument_count = match command.as_str() {
        "list" | "watch" | "clear" => 2,
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

fn check_date_time(date_time: &str) -> bool {
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
