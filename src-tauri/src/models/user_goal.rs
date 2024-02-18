use sqlx::FromRow; 
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, FromRow)] 
pub struct UserGoal {
    pub id: i32, 
    pub weight: f64,
    pub protein: f64, 
    pub carbohydrate: f64, 
    pub fat: f64, 
    pub calories: f64
}

impl UserGoal {
    pub fn new(weight: f64, protein: f64, carbohydrate: f64, fat: f64, calories: f64) -> Self {
        UserGoal {
            id: 0, 
            weight, 
            protein, 
            carbohydrate, 
            fat, 
            calories
        }
    }
}