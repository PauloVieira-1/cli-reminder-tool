use dirs;
use std::path::PathBuf;
use crate::reminder::Reminder;
use colored::Colorize;


fn get_data_file_path() -> PathBuf {
    let data_dir = dirs::config_dir().expect("Could not determine config directory");
    // println!("Data directory: {}", data_dir.display());
    let reminders_dir = data_dir.join("cli_reminder_tool");
    if !reminders_dir.exists() {
        std::fs::create_dir_all(&reminders_dir).expect("Could not create reminders directory");
    }
    reminders_dir.join("reminders.json")
}

pub fn save_reminder_to_file(reminder: &Reminder) {
    
    if !check_date_time(reminder) {
        return;
    }

    let file_path = get_data_file_path();
    let mut reminders: Vec<Reminder> = vec![];

    if file_path.exists() {
        let data = std::fs::read_to_string(&file_path).expect("Could not read data file");
        match serde_json::from_str(&data) {
            Ok(r) => reminders = r,
            Err(_) => reminders = vec![], 
        }
    } 

    reminders.push(reminder.clone());

    write_to_file(&reminders, &file_path).expect("Could not write to data file");
    println!("{}", format!("Reminder saved successfully!").green());

}

pub fn get_remdiners() -> Vec<Reminder> {
    let file_path = get_data_file_path();
    if !file_path.exists() {
        return vec![];
    }

    let data = std::fs::read_to_string(&file_path).expect("Could not read data file");
    serde_json::from_str(&data).expect("Could not parse data file")

}

pub fn update_reminders(id: i32, title: String, description: String, due_date: String, timestamp: String) -> Result<(), Box<dyn std::error::Error>> {
    

    let reminder = Reminder::new(id, title.clone(), description.clone(), due_date.clone(), timestamp.clone());
    
    if !check_date_time(&reminder) {
        return Ok(());
    }
    
    let file_path = get_data_file_path();
    let mut reminders: Vec<Reminder> = if file_path.exists() {
        let data = std::fs::read_to_string(&file_path)?;
        serde_json::from_str(&data)?
    } else {
        vec![]
    };

    if let Some(r) = reminders.iter_mut().find(|r| r.id == id) {
    r.title = title;
    r.description = description;
    r.due_date = due_date;
    r.timestamp = timestamp;
    println!("{}", format!("Reminder with ID {} updated successfully!", id).green());
    } else {
        println!("{}", "No reminder found with the specified ID.".red());
        return Ok(());
    }


    write_to_file(&reminders, &file_path)?;

    Ok(())
}


fn check_date_time(reminder: &Reminder) -> bool {
    let reminder_datetime = chrono::NaiveDateTime::parse_from_str(
                &format!("{} {}", reminder.due_date, reminder.timestamp),
                "%Y-%m-%d %H:%M"
            );

    let currrent_time = chrono::Utc::now().naive_utc();
    if let Ok(due_time) = reminder_datetime {
        if due_time < currrent_time {
            println!("{}", "Due date and time must be in the future.".red());
            return false;
        }
    }
    true
}


pub fn remove_reminder(id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = get_data_file_path();
    let mut reminders: Vec<Reminder> = vec![];

    if file_path.exists() {
        let data = std::fs::read_to_string(&file_path).expect("Could not read data file");
        reminders = serde_json::from_str(&data).expect("Could not parse data file");
    }

    if reminders.iter().all(|r| r.id != id) { 
        println!("{}", "No reminders found with the specified ID.".red());
        return Ok(());
    }

    reminders.retain(|r| r.id != id);

    write_to_file(&reminders, &file_path).expect("Could not write to data file");
    println!("{}", format!("Reminder with ID {} removed successfully!", id).green());

    Ok(())
}

pub fn clear_reminders() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = get_data_file_path();
    if file_path.exists() {
        std::fs::remove_file(&file_path).expect("Failed to clear reminders");
    } 
    Ok(())
}

fn write_to_file(data: &Vec<Reminder>, file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {

    let serialized_data = serde_json::to_string(&data).expect("Could not serialize reminders");
    std::fs::write(file_path, serialized_data).expect("Could not write to data file");

    Ok(())

}