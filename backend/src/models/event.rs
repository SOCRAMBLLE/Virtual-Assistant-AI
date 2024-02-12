use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use serde_json::Deserializer;

#[derive(Debug)]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    // Adicione outros campos conforme necessário
}
