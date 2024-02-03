use chrono::NaiveDateTime;
use sqlx::FromRow; 
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, FromRow)] 
pub struct Meal {
    pub id: i32, 
    pub log_id: i32, 
    pub name: String, 
    pub entry_timestamp: NaiveDateTime, 
}

impl Meal {
    pub fn new(id: i32, log_id: i32, name: String, entry_timestamp: NaiveDateTime) -> Self {
        Self { id, log_id, name, entry_timestamp }
    }
}