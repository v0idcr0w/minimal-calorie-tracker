use sqlx::{FromRow, types::chrono::NaiveDate}; 
use serde::{Serialize, Deserialize};
use super::macros_total::MacrosTotal;

#[derive(Debug, PartialEq, Serialize, Deserialize, FromRow)] 
pub struct DailyLog {
    pub id: i32, 
    pub weight: f64,
    pub total_protein: f64, 
    pub total_carbohydrate: f64, 
    pub total_fat: f64, 
    pub total_calories: f64,
    pub entry_date: NaiveDate, 
}


impl DailyLog {
    pub fn new(entry_date: NaiveDate) -> Self {
        Self { id: 0, weight: 0.0, total_protein: 0.0, total_carbohydrate: 0.0, total_fat: 0.0, total_calories: 0.0, entry_date }
    }
    pub fn update_macros(mut self, macros: MacrosTotal) -> Self {
        self.total_protein = macros.protein; 
        self.total_carbohydrate = macros.carbohydrate; 
        self.total_fat = macros.fat; 
        self.total_calories = macros.calories; 
        self 
    }
}