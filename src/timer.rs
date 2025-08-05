use chrono::{DateTime, Utc};
use tokio::time;
use std::time::Duration;
use std::process::Command;

/// Creates a native notification on the user's system with the provided reminder data.
/// 
/// The reminder data is expected to be a tuple of two strings, where the first string is the title
/// of the reminder and the second string is the description of the reminder.
/// 
/// On macOS, the `osascript` command is used to execute the AppleScript command to display a
/// notification.
/// 
/// If the command fails to execute, an error is returned.

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


/// Creates a native notification on the user's system with the provided reminder data.
/// 
/// The reminder data is expected to be a tuple of two strings, where the first string is the title
/// of the reminder and the second string is the description of the reminder.
/// 
/// If the command fails to execute, an error is returned.

#[cfg(target_os = "windows")]
pub fn create_notification(reminder_data: [String; 2]) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("powershell")
        .arg("-Command")
        .arg(&format!(
            "[System.Windows.MessageBox]::Show('{}', '{}')",
            reminder_data[1], reminder_data[0]
        ))
        .output()?;
    Ok(())
}


