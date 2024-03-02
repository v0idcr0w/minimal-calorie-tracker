use sqlx::FromRow; 
use serde::{Serialize, Deserialize};
use super::{food_normalized::FoodNormalized, macros_total::MacrosTotal};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)] 
pub struct Ingredient {
    pub id: i32, 
    pub recipe_id: i32, 
    pub food_normalized_id: i32, 
    pub name: String,
    pub amount: f64, 
    pub unit: String, 
    pub protein: f64,
    pub carbohydrate: f64,
    pub fat: f64,
    pub calories: f64
}

impl Ingredient {

    pub fn from(food_normalized: FoodNormalized, recipe_id: i32, amount: f64) -> Self {
        // initializes a new instance of Ingredient by taking some amount and multiplying macros by the correct value 
        let multiplier = amount / food_normalized.serving_size; 
        Self { 
            id: 0, // set id to zero 
            recipe_id,
            food_normalized_id: food_normalized.id, 
            name: food_normalized.name, 
            amount, 
            unit: food_normalized.unit, 
            protein: food_normalized.normalized_protein * multiplier, 
            carbohydrate: food_normalized.normalized_carbohydrate * multiplier, 
            fat: food_normalized.normalized_fat * multiplier, 
            calories: food_normalized.normalized_calories * multiplier 
        } 
    }

    pub fn into_macros_total(&self) -> MacrosTotal {
        // converts the ingredient into a MacrosTotal instance 
        MacrosTotal::new(self.protein, self.carbohydrate, self.fat, self.calories)
    }
}