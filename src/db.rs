use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use surrealdb::types::{Datetime, RecordId, SurrealValue, Uuid};

#[derive(SurrealValue, Serialize, Deserialize, Debug)]
pub struct CalendarEventFull {
    pub id: RecordId,
    pub name: String,
    pub summary: String,
    pub themes: HashSet<String>,
    pub article_url: String,
    pub timestamp: Datetime,
}

#[derive(SurrealValue, Serialize, Deserialize, Debug)]
pub struct SummaryBucketFull {
    pub id: Uuid,
    pub summary: String,
    pub begin_timestamp: Datetime,
    pub end_timestamp: Datetime,
}

#[derive(SurrealValue, Serialize, Deserialize, Debug)]
pub struct Theme {
    pub name: String,
    pub event_count: u32,
}