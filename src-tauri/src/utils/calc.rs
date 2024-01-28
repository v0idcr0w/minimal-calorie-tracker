use sqlx::SqlitePool; 
use crate::models::{food::Food, food_normalized::FoodNormalized, meal::Meal, daily_log::DailyLog}; 

#[derive(Debug)]
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

pub async fn compute_meal_total(meal_id: i32, pool: &SqlitePool) -> MacrosTotal {
    let foods: Vec<Food> = Meal::get_foods_by_id(meal_id, &pool).await.unwrap(); 

    let mut protein = 0.0; 
    let mut carbohydrate = 0.0;
    let mut fat = 0.0; 
    let mut calories = 0.0; 

    for food in foods {
        protein += food.protein; 
        carbohydrate += food.carbohydrate; 
        fat += food.fat; 
        calories += food.calories;     
    }

    MacrosTotal::new(protein, carbohydrate, fat, calories)

}

pub async fn compute_daily_totals(meal_ids: &[i32], pool: &SqlitePool) -> MacrosTotal {
    // compute macros for each meal in the list of meal ids 
    let mut protein = 0.0; 
    let mut carbohydrate = 0.0;
    let mut fat = 0.0; 
    let mut calories = 0.0; 

    for &meal_id in meal_ids {
        let meal_total = compute_meal_total(meal_id, pool).await;
        protein += meal_total.protein; 
        carbohydrate += meal_total.carbohydrate; 
        fat += meal_total.fat; 
        calories += meal_total.calories; 
    }
    MacrosTotal::new(protein, carbohydrate, fat, calories)
}