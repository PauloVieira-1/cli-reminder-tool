
use rand::Rng;
use crate::reminder::{Reminder, CommandType};
use crate::data_manager::{save_reminder_to_file, get_remdiners, remove_reminder, update_reminders};

pub fn add_command(args: &[String]) {
    println!("Adding a new reminder...");
    let id = rand::thread_rng().gen_range(1..100000);
    let reminder = Reminder::new(id, args[2].clone(), args[3].clone(), args[4].clone(), args[5].clone());
    save_reminder_to_file(&reminder);
}


pub fn list_reminders() {
    let reminders = get_remdiners();
    if reminders.is_empty() {
        println!("No reminders found.");
        return;
    }

    let longest_length: usize= get_longest_vector_length(&reminders);

    println!("\n\n######################");
    println!("Listing all reminders:");
    println!("######################\n");

     println!("{:-^1$}", "", longest_length);
            for r in reminders {
                println!("ID: {}, Title: {}, Description: {}, Due Date: {}, Time Stamp: {}", r.id, r.title, r.description, r.due_date, r.timestamp);
            println!("{:-^1$}", "", longest_length);
            }
            
            println!("\n")
}

pub fn remove_command(args: &[String]) {
    println!("Removing a reminder...");
    let id: i32 = args[2].parse().expect("Invalid ID format");
    remove_reminder(id).expect("Failed to remove reminder");
}

pub fn update_command(args: &[String]) {
    println!("Updating a reminder...");
    update_reminders(
        args[2].parse().expect("Invalid ID format"),
        args[3].clone(),
        args[4].clone(),
        args[5].clone(),
    ).expect("Failed to update reminder");
    println!("Reminder updated successfully!");
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

