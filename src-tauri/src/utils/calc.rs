use sqlx::SqlitePool; 
use crate::models::{food::Food, ingredient::Ingredient, macros_total::MacrosTotal, meal::Meal}; 


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

pub async fn compute_recipe_total(recipe_id: i32, pool: &SqlitePool) -> MacrosTotal {
    let ingredients: Vec<Ingredient> = Ingredient::get_by_recipe_id(recipe_id, &pool).await.unwrap(); 

    let mut protein = 0.0; 
    let mut carbohydrate = 0.0;
    let mut fat = 0.0; 
    let mut calories = 0.0; 

    for ingredient in ingredients {
        protein += ingredient.protein; 
        carbohydrate += ingredient.carbohydrate; 
        fat += ingredient.fat; 
        calories += ingredient.calories;     
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