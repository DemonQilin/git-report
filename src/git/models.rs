use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Commit {
    pub message: String,
    pub author: String,
    pub time: DateTime<Utc>,
    pub changes: Vec<String>,
}
