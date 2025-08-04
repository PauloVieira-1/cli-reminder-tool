use chrono::Utc;
use tokio::time::{sleep, Duration};
use crate::data_manager::{get_remdiners, remove_reminder};
use crate::timer::create_notification;

pub async fn watch_reminders() -> Result<(), Box<dyn std::error::Error>> {
    println!("📅 Reminder watcher started...");

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
                    println!("🔔 Reminder: {} - {}", reminder.title, reminder.description);
                    create_notification([reminder.title.clone(), reminder.description.clone()])?; 
                    remove_reminder(reminder.id)?; 
                }
            }
        }

        sleep(Duration::from_secs(60)).await; 
    }
}
