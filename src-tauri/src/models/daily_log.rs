use sqlx::{FromRow, types::chrono::NaiveDateTime}; 
use serde::{Serialize, ser::SerializeStruct};

#[derive(Debug, PartialEq, FromRow)] 
pub struct DailyLog {
    pub id: i32, 
    pub weight: f64,
    pub total_protein: f64, 
    pub total_carbohydrate: f64, 
    pub total_fat: f64, 
    pub total_calories: f64,
    pub current_date: NaiveDateTime, 
}

// must implement Serialize manually. 
impl Serialize for DailyLog {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
                let formatted_date = self.current_date.format("%Y-%m-%d %H:%M:%S").to_string();
                let mut state = serializer.serialize_struct("DailyLog", 7)?;
                state.serialize_field("id", &self.id)?;
                state.serialize_field("weight", &self.weight)?;
                state.serialize_field("total_protein", &self.total_protein)?;
                state.serialize_field("total_carbohydrate", &self.total_carbohydrate)?;
                state.serialize_field("total_fat", &self.total_fat)?;
                state.serialize_field("total_calories", &self.total_calories)?;
                state.serialize_field("current_date", &formatted_date)?;
                state.end()
    }
}

impl DailyLog {
    pub fn new(id: i32, weight: f64, total_protein: f64, total_carbohydrate: f64, total_fat: f64, total_calories: f64, current_date: NaiveDateTime) -> Self {
        Self { id, weight, total_protein, total_carbohydrate, total_fat, total_calories, current_date }
    }
}