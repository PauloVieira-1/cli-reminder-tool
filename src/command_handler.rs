
use rand::Rng;
use crate::data_manager::{update_reminders, clear_reminders, remove_reminder, get_remdiners, save_reminder_to_file};
use serde::{Serialize, Deserialize};
use colored::Colorize;
use crate::reminder::Reminder;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CommandType {
    Add,
    List,
    Remove,
    Update,
    Watch,
    Clear,
}

pub fn add_command(args: &[String]) {
    println!("{}", format!("\nAdding a new reminder...").blue());
    let id = rand::rng().random_range(1..100000);
    let reminder = Reminder::new(id, args[2].clone(), args[3].clone(), args[4].clone(), args[5].clone());
    save_reminder_to_file(&reminder);
}


pub fn list_reminders() {
    let reminders = get_remdiners();
    if reminders.is_empty() {
        println!("{}", format!("No reminders found.").red());
        return;
    }

    let longest_length: usize= get_longest_vector_length(&reminders);

    println!("{}", format!("\n--------------------------").blue());
    println!("{}", format!("| Listing all reminders: |").blue());
    println!("{}", format!("--------------------------\n").blue());

     println!("{:-^1$}", "", longest_length);
            for r in reminders {
                println!("ID: {}, Title: {}, Description: {}, Due Date: {}, Time Stamp: {}", r.id, r.title, r.description, r.due_date, r.timestamp);
            println!("{:-^1$}", "", longest_length);
            }
            
            println!("\n")
}

pub fn remove_command(args: &[String]) {
    println!("{}", format!("\nRemoving a reminder...").blue());
    let id: i32 = args[2].parse().expect("Invalid ID format");
    remove_reminder(id).expect("Failed to remove reminder");
}

pub fn update_command(args: &[String]) {
    println!("{}", format!("\nUpdating a reminder...").blue());
    update_reminders(
        args[2].parse().expect("Invalid ID format"),
        args[3].clone(),
        args[4].clone(),
        args[5].clone(),
        args[6].clone()
    ).expect("Failed to update reminder");
}


pub fn get_longest_vector_length(vectors: &Vec<Reminder>) -> usize {

    let mut longest_length = 0;

    for reminder in vectors {
        let reminder_string = format!("ID: {}, Title: {}, Description: {}, Due Date: {}, Time Stamp: {}", reminder.id, reminder.title, reminder.description, reminder.due_date, reminder.timestamp);
        if reminder_string.len() > longest_length {
            longest_length = reminder_string.len(); 
        }
    }
    longest_length
}

pub fn clear_command(){
    println!("{}", format!("\nClearing all reminders...").blue());
    clear_reminders().expect("Failed to clear reminders");
    println!("{}", format!("Reminders cleared successfully!").green());   
}
