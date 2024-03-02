use sqlx::FromRow; 
use serde::{Serialize, Deserialize}; 
use super::{food_normalized::FoodNormalized, recipe::Recipe}; 
#[derive(Debug, Serialize, Deserialize, PartialEq, FromRow)] 
pub struct Food {
   pub id: i32, 
   pub food_normalized_id: Option<i32>, 
   pub recipe_id: Option<i32>, 
   pub meal_id: i32, 
   pub name: String, 
   pub unit: String,
   pub amount: f64, 
   pub protein: f64, 
   pub carbohydrate: f64,
   pub fat: f64, 
   pub calories: f64 
}

impl Food {

    pub fn from_food_normalized(food_normalized: FoodNormalized, meal_id: i32, amount: f64) -> Self {
        // initializes a new instance of Food by taking some amount and multiplying macros by the correct value 
        let multiplier = amount / food_normalized.serving_size; 
        Self { 
            id: 0, // set id to zero 
            food_normalized_id: Some(food_normalized.id), 
            recipe_id: None,  // null recipe id 
            meal_id,
            name: food_normalized.name, 
            unit: food_normalized.unit, 
            amount, 
            protein: food_normalized.normalized_protein * multiplier, 
            carbohydrate: food_normalized.normalized_carbohydrate * multiplier, fat: food_normalized.normalized_fat * multiplier, 
            calories: food_normalized.normalized_calories * multiplier } 
    }

    pub fn from_recipe(recipe: Recipe, meal_id: i32, amount: f64) -> Self {
        // initializes a new instance of Food by taking some amount and multiplying macros by the correct value 
        let multiplier = amount / recipe.serving_size; 
        Self { 
            id: 0, // set id to zero 
            food_normalized_id: None, 
            recipe_id: Some(recipe.id), 
            meal_id,
            name: recipe.name, 
            unit: recipe.unit, 
            amount, 
            protein: recipe.protein * multiplier, 
            carbohydrate: recipe.carbohydrate * multiplier, 
            fat: recipe.fat * multiplier, 
            calories: recipe.calories * multiplier 
        } 
    }
}