use chrono::{serde::ts_milliseconds_option, DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod post;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub description: String,
    pub content: String,
    pub tags: Vec<String>,

    #[serde(with = "ts_milliseconds_option")]
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}
