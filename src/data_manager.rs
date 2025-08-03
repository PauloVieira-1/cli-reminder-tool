use dirs;
use std::path::PathBuf;
use crate::reminder::Reminder;

fn get_data_file_path() -> PathBuf {
    let data_dir = dirs::config_dir().expect("Could not determine config directory");
    println!("Data directory: {}", data_dir.display());
    let reminders_dir = data_dir.join("cli_reminder_tool");
    if !reminders_dir.exists() {
        std::fs::create_dir_all(&reminders_dir).expect("Could not create reminders directory");
    }
    reminders_dir.join("reminders.json")
}

pub fn save_reminder_to_file(reminder: &Reminder) {
    let file_path = get_data_file_path();
    let mut reminders: Vec<Reminder> = vec![];

    if file_path.exists() {
        let data = std::fs::read_to_string(&file_path).expect("Could not read data file");
        reminders = serde_json::from_str(&data).expect("Could not parse data file");
    }

    reminders.push(reminder.clone());
    
    let serialized_data = serde_json::to_string(&reminders).expect("Could not serialize reminders");
    std::fs::write(file_path, serialized_data).expect("Could not write to data file");

    println!("Reminder saved successfully!");
}
