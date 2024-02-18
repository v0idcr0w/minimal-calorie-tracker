use sqlx::FromRow; 
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, FromRow)] 
pub struct FoodNormalized {
    pub id: i32, 
    pub name: String, 
    pub serving_size: f64, 
    pub unit: String, 
    pub normalized_protein: f64, 
    pub normalized_carbohydrate: f64, 
    pub normalized_fat: f64, 
    pub normalized_calories: f64
}

#[derive(Debug, Deserialize, Serialize)] 
pub struct FoodNormalizedCsv {
    // this struct is the same as the previous struct, but it's used to serialize/deserialize the CSV file, and doesn't have an id. 
    pub name: String, 
    pub serving_size: f64, 
    pub unit: String, 
    pub normalized_protein: f64, 
    pub normalized_carbohydrate: f64, 
    pub normalized_fat: f64, 
    pub normalized_calories: f64    
}

impl FoodNormalized {
    pub fn new(id: i32, name: String, serving_size: f64, unit: String, normalized_protein: f64, normalized_carbohydrate: f64, normalized_fat: f64, normalized_calories: f64) -> Self {
        Self { id, name, serving_size, unit, normalized_protein, normalized_carbohydrate, normalized_fat, normalized_calories } 
    }
}

impl From<FoodNormalized> for FoodNormalizedCsv {
    fn from(food: FoodNormalized) -> Self {
        Self { name: food.name, serving_size: food.serving_size, unit: food.unit, normalized_protein: food.normalized_protein, normalized_carbohydrate: food.normalized_carbohydrate, normalized_fat: food.normalized_fat, normalized_calories: food.normalized_calories }
    }
}