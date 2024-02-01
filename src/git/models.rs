use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Commit {
    pub message: String,
    pub author: String,
    #[serde(with = "ts_seconds")]
    pub time: DateTime<Utc>,
    #[serde(default)]
    pub changes: Vec<String>,
}
