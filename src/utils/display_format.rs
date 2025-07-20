use super::types::{PriorityType, Project, Task};
use colored::*;
use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, NaiveTime, TimeZone, Timelike, Utc, Weekday};
use chrono::format::ParseError;
use serde_json::Value;

// Pretty print TASKS
pub fn print_task (task: &Task, show_proj: bool) -> Result<(), Box<dyn std::error::Error>> {
    let id = format!("[{}]", task.id).yellow();
    let status = if task.is_done { "[]".green() } else { "[ ]".red() };

    let priority_dot = match task.priority {
        Some(PriorityType::High) => " ".red(),
        Some(PriorityType::Medium) => " ".yellow(),
        Some(PriorityType::Low) => " ".green(),
        None => " ".white().dimmed(),
    };

    let due_str = task.due.as_deref().unwrap_or("");
    let due = format_pretty_date(due_str)?;

    if show_proj {
        let pid = format!("<{}>", task.project_id).blue();
        println!("{} {} {} {:<35} {} {}\n", id, status, pid, task.title, priority_dot, due.blue());
    } else {
        println!("{} {} {:<35} {} {}\n", id, status, task.title, priority_dot, due.blue());
    }

    if let Some(desc) = &task.description {
        println!("{} {}\n", "-".yellow(), desc);
    }

    Ok(())
}

// Pretty print PROJECTS
pub fn print_project (project: Project) {
    let pid = format!("[{}]", project.id).yellow();
    println!("{} {} {:<35}\n", " ".yellow(), pid, project.name);
}

// Pretty print ERRORS
pub fn log_err (res: Value) {
    if let Some(msg) = res.get("message").and_then(|m| m.as_str()) {
        println!("{} {}", " ".red(), msg.red());
    } else if let Some(err) = res.get("error").and_then(|e| e.as_str()) {
        println!("{} The following error occured: {}", " ".red(), err.red());
    }
}

// Custom formatter for DATETIME
pub fn natural_to_datetime (input: &str) -> Option<String> {
    let input = input.trim().to_lowercase();
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2 {
        return None;
    }

    let day_or_date = parts[0];
    let time_str = parts[1];
    let time = NaiveTime::parse_from_str(time_str, "%H:%M").ok()?;

    let today = Local::now().date_naive();

    let due_date = match day_or_date {
        "today" => today,
        "tomorrow" => today + Duration::days(1),
        _ => {
            if let Ok(date) = NaiveDate::parse_from_str(day_or_date, "%d-%d-%Y") {
                date
            } else {
                let weekday = match day_or_date {
                    "monday" => Weekday::Mon,
                    "tuesday" => Weekday::Tue,
                    "wednesday" => Weekday::Wed,
                    "thursday" => Weekday::Thu,
                    "friday" => Weekday::Fri,
                    "saturday" => Weekday::Sat,
                    "sunday" => Weekday::Sun,
                    _ => return None,
                };

                let cur_weekday = today.weekday();
                let mut days_ahead = (weekday.num_days_from_monday() + 7 - cur_weekday.num_days_from_monday()) % 7;

                if days_ahead == 0 {
                    days_ahead = 7;
                }

                today + Duration::days(days_ahead as i64)
            }
        }
    };

    let local_dt = Local.with_ymd_and_hms(due_date.year(), due_date.month(), due_date.day(), time.hour(), time.minute(), 0).unwrap();
    let utc_dt: DateTime<Utc> = local_dt.with_timezone(&Utc);

    Some(utc_dt.to_rfc3339())
}

// HELPER: Add suffices like 1st, 2nd, 4th, etc.
fn ordinal_suffix(n: u32) -> &'static str {
    match n {
        11 | 12 | 13 => "th",
        _ => match n % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    }
}

// Normalize date format
pub fn format_pretty_date (input: &str) -> Result<String, ParseError> {
    if input.trim().is_empty() {
        return Ok("No due date".to_string());
    }

    let dt_utc: DateTime<chrono::FixedOffset> = input.parse()?;
    let dt: DateTime<Local> = dt_utc.with_timezone(&Local);

    let day = dt.day();
    let suffix = ordinal_suffix(day);
    let month = dt.format("%B").to_string();
    let year = dt.year();
    let time = dt.format("%H:%M").to_string();

    Ok(format!("{}{} {}, {}", day, suffix, month, year) + "    " + &time)
}


// Get expiry date of jwt
// pub fn expiry_jwt ()
