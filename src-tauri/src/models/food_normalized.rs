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

impl FoodNormalized {
    pub fn new(id: i32, name: String, serving_size: f64, unit: String, normalized_protein: f64, normalized_carbohydrate: f64, normalized_fat: f64, normalized_calories: f64) -> Self {
        Self { id, name, serving_size, unit, normalized_protein, normalized_carbohydrate, normalized_fat, normalized_calories } 
    }
}

