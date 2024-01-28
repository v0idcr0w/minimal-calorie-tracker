use sqlx::SqlitePool;
use sqlx::types::chrono;
use crate::models::{food::Food, food_normalized::FoodNormalized, meal::Meal, daily_log::DailyLog}; 
use crate::utils::calc;  

#[sqlx::test(fixtures("init_tables", "prepopulate"))]  
async fn test_edit_food_normalized(pool: SqlitePool) -> sqlx::Result<()> {
    let mut query = FoodNormalized::get_by_id(2, &pool).await?; 

    let banana = FoodNormalized::new(2, "banana".to_string(), 118.0, "g".to_string(), 1.3, 27.0, 0.4, 105.0); 

    query.update(banana.clone()).await; 

    query.update_entry(&pool).await?; 

    let query_from_db = FoodNormalized::get_by_id(2, &pool).await?; 

    assert_eq!(banana, query_from_db); 

    Ok(())
}

#[sqlx::test(fixtures("init_tables", "prepopulate", "foods"))] 
async fn test_edit_food(pool: SqlitePool) -> sqlx::Result<()> {
    let mut food = Food::get_by_id(1, &pool).await?;  

    food.update(100.0).await;

    food.update_entry(&pool).await?; 

    let food_from_db = Food::get_by_id(1, &pool).await?; 

    assert_eq!(food, food_from_db);  

    Ok(())
}

#[sqlx::test(fixtures("init_tables", "prepopulate", "foods", "meals"))] 
async fn test_edit_meal(pool: SqlitePool) -> sqlx::Result<()> {
    let mut meal = Meal::get_by_id(1, &pool).await?; 
    meal.update("lunch".to_string()).await; 
    meal.update_entry(&pool).await?; 
    let meal_from_db = Meal::get_by_id(1, &pool).await?; 
    assert_eq!(meal, meal_from_db); 
    Ok(())
}

#[sqlx::test(fixtures("init_tables", "prepopulate", "foods", "meals"))] 
async fn test_edit_daily_log(pool: SqlitePool) -> sqlx::Result<()> {
    let total_macros = calc::compute_daily_totals(&[1, 2], &pool).await;

    let now = chrono::Local::now().naive_local(); 

    let mut daily_log = DailyLog::new(0, 59.6, total_macros.protein, total_macros.carbohydrate, total_macros.fat, total_macros.calories, now); 

    daily_log.create_entry(&pool).await?; 
    daily_log.update(55.5).await; 
    daily_log.update_entry(&pool).await?; 

    let log_from_db = DailyLog::get_by_id(1, &pool).await?; 

    assert_eq!(daily_log, log_from_db); 

    Ok(())
}