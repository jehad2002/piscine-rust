use chrono::{NaiveDateTime, Datelike};
use json::JsonValue;
use std::collections::HashMap;

#[derive(Debug)]
pub struct CommitData {
    pub author: String,
    pub date: String,
}

pub fn commits_per_week(data: &JsonValue) -> HashMap<String, u32> {
    let mut commits_by_week: HashMap<String, u32> = HashMap::new();

    for commit in data.members() {
        if let Some(commit_date) = commit["commit"]["author"]["date"].as_str() {
            // Parse the commit date string to NaiveDateTime
            if let Ok(date) = NaiveDateTime::parse_from_str(commit_date, "%+") {
                // Get the year and week number (ISO week)
                let year = date.year();
                let week = date.iso_week().week();
                let week_str = format!("{}-W{}", year, week);
                
                // Increment the commit count for the corresponding week
                *commits_by_week.entry(week_str).or_insert(0) += 1;
            }
        }
    }

    commits_by_week
}

pub fn commits_per_author(data: &JsonValue) -> HashMap<String, u32> {
    let mut commits_by_author: HashMap<String, u32> = HashMap::new();

    for commit in data.members() {
        if let Some(author) = commit["author"]["login"].as_str() {
            // Increment the commit count for the author
            *commits_by_author.entry(author.to_string()).or_insert(0) += 1;
        }
    }

    commits_by_author
}
