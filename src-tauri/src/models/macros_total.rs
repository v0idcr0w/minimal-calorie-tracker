use serde::{Serialize, Deserialize}; 
#[derive(Debug, Serialize, Deserialize)]
pub struct MacrosTotal {
    pub protein: f64, 
    pub carbohydrate: f64, 
    pub fat: f64, 
    pub calories: f64
}

impl MacrosTotal {
    pub fn new(protein: f64, carbohydrate: f64, fat: f64, calories: f64) -> Self {
        Self { protein, carbohydrate, fat, calories }
    }
}