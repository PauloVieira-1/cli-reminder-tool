use cli_reminder_tool::reminder;
use reminder::Reminder; 

fn main() {
    
    let my_reminder = Reminder::new(
        1,
        String::from("Test Reminder"),
        String::from("This is a test reminder."),
        String::from("2023-10-01"),
    );

    println!(
        "Reminder ID: {}, Title: {}, Description: {}, Due Date: {}", 
        my_reminder.id, my_reminder.title, my_reminder.description, my_reminder.due_date
    );
}
