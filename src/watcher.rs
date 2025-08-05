use chrono::Utc;
use tokio::time::{sleep, Duration};
use crate::data_manager::{get_remdiners, remove_reminder};
use crate::timer::create_notification;
use colored::Colorize;

/// Watches the reminders and triggers a notification when the due date is reached.
///
/// This function runs an infinite loop that every 60 seconds checks if the due date
/// of any reminder is reached. If so, it triggers a notification and removes the reminder.
///
/// The function can be used with the tokio runtime to run the loop concurrently with
/// other tasks.
///
/// The function returns a Result to handle any errors that may occur while running the loop.
pub async fn watch_reminders() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", format!("\n📅 Reminder watcher started...").blue());

    loop {
        let reminders = get_remdiners();
        let now = Utc::now();

        for reminder in reminders {
            let reminder_datetime = chrono::NaiveDateTime::parse_from_str(
                &format!("{} {}", reminder.due_date, reminder.timestamp),
                "%Y-%m-%d %H:%M"
            );

            if let Ok(due_time) = reminder_datetime {
                if due_time <= now.naive_utc() {
                    println!("{}", format!("🔔 Reminder: {} - {}", reminder.title, reminder.description).yellow()); 
                    create_notification([reminder.title.clone(), reminder.description.clone()])?; 
                    remove_reminder(reminder.id)?; 
                }
            }
        }

        sleep(Duration::from_secs(60)).await; 
    }
}
