use chrono::{DateTime, Utc};
use tokio::time;
use std::time::Duration;
use std::process::Command;

// pub async fn start_timer(datetime: String) -> Result<(), Box<dyn std::error::Error>> {

//     let current_time: DateTime<Utc> = Utc::now();
//     let due_date = DateTime::parse_from_str(&datetime, "%Y-%m-%d %H:%M")?;

//     if due_date < current_time {
//         eprintln!("Due date is in the past. Please provide a future date.");
//         return Ok(());
//     }

//     let duration = due_date.signed_duration_since(current_time).to_std()?;
//     println!("Timer set for {} seconds.", duration.as_secs());
//     time::sleep(Duration::from_secs(duration.as_secs())).await;
//     println!("Time's up! Reminder for: {}", datetime);
//     create_notification()?;
//     Ok(())

// }

#[cfg(target_os = "macos")]

pub fn create_notification(reminder_data: [String; 2]) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("osascript")
        .arg("-e")
        .arg(&format!(
            "display notification \"{}\" with title \"{}\"",
            reminder_data[1], reminder_data[0]
        ))
        .output()?;
    Ok(())
}

