use sqlx::FromRow; 
use serde::{Serialize, Deserialize}; 
use super::food_normalized::FoodNormalized; 
#[derive(Debug, Serialize, Deserialize, PartialEq, FromRow)] 
pub struct Food {
   pub id: i32, 
   pub foods_normalized_id: i32, 
   pub name: String, 
   pub unit: String,
   pub amount: f64, 
   pub protein: f64, 
   pub carbohydrate: f64,
   pub fat: f64, 
   pub calories: f64 
}

impl Food {
    pub fn new(id: i32, foods_normalized_id: i32, name: String, unit: String, amount: f64, protein: f64, carbohydrate: f64, fat: f64, calories: f64) -> Self {
        Self { id, foods_normalized_id, name, unit, amount, protein, carbohydrate, fat, calories } 
    }

    pub fn from(food_normalized: FoodNormalized, amount: f64) -> Self {
        // initializes a new instance of Food by taking some amount and multiplying macros by the correct value 
        let multiplier = amount / food_normalized.serving_size; 
        Self { 
            id: 0, // set id to zero 
            foods_normalized_id: food_normalized.id, 
            name: food_normalized.name, 
            unit: food_normalized.unit, 
            amount, 
            protein: food_normalized.normalized_protein * multiplier, 
            carbohydrate: food_normalized.normalized_carbohydrate * multiplier, fat: food_normalized.normalized_fat * multiplier, 
            calories: food_normalized.normalized_calories * multiplier } 
    }
}