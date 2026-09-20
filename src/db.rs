use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use surrealdb::types::{Datetime, SurrealValue, Uuid};

#[derive(SurrealValue, Serialize, Deserialize, Debug)]
pub struct CalendarEventFull {
    pub id: String,
    pub name: String,
    pub summary: String,
    pub themes: HashSet<String>,
    /// similarity score per theme, keyed by theme name
    pub similarities: HashMap<String, f64>,
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

#[derive(SurrealValue, Serialize, Deserialize, Debug)]
pub struct BucketWithSources {
    pub id: String,
    pub summary: String,
    pub theme: String,
    pub begin_timestamp: Datetime,
    pub end_timestamp: Datetime,
    /// event keys, ordered by similarity to `theme` (highest first)
    pub sources: Vec<String>,
}