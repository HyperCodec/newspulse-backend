use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use surrealdb::types::{Datetime, SurrealValue};

#[derive(SurrealValue, Serialize, Deserialize, Debug)]
pub struct CalendarEventFull {
    pub id: String,
    pub name: String,
    pub summary: String,
    pub themes: HashSet<String>,
    pub similarities: HashMap<String, f64>,
    pub article_url: String,
    pub timestamp: Datetime,
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
    pub sources: Vec<String>,
}

#[derive(SurrealValue, Serialize, Deserialize, Debug)]
pub struct ExtraData {
    pub id: String,
    pub extra: serde_json::Value,
}
