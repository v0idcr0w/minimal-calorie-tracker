use sqlx::SqlitePool; 
use crate::models::{food::Food, macros_total::MacrosTotal}; 

pub fn compute_meal_total(foods: Vec<Food>) -> MacrosTotal {

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
        let foods = Food::get_by_meal_id(meal_id, pool).await.unwrap();
        let meal_total = compute_meal_total(foods);
        protein += meal_total.protein; 
        carbohydrate += meal_total.carbohydrate; 
        fat += meal_total.fat; 
        calories += meal_total.calories; 
    }
    MacrosTotal::new(protein, carbohydrate, fat, calories)
}