use chrono::{DateTime, Local, Datelike, Timelike};

pub fn get_human_date(timestamp: DateTime<Local>) -> String {
    format!("{:02}/{:02}/{} à {:02}:{:02}:{:02}", timestamp.day(), timestamp.month(), timestamp.year(), timestamp.hour(), timestamp.minute(), timestamp.second())
}