use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DateAt(DateTime<Utc>);

impl DateAt {
    pub fn now() -> Self {
        DateAt(Utc::now()) // Example: 2024-01-21T11:05:49.639466247Z
    }
}