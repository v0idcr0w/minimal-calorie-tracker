use sqlx::FromRow; 
use serde::{Serialize, Deserialize};


#[derive(Debug, PartialEq, Serialize, Deserialize, FromRow)] 
pub struct Recipe {
    pub id: i32, 
    pub name: String, 
    pub serving_size: f64, 
    pub unit: String, 
    pub protein: f64, 
    pub carbohydrate: f64, 
    pub fat: f64, 
    pub calories: f64
}

impl Recipe {
    pub fn new(id: i32, name: String, serving_size: f64, unit: String, protein: f64, carbohydrate: f64, fat: f64, calories: f64) -> Self {
        Self { id, name, serving_size, unit, protein, carbohydrate, fat, calories }
    }
}