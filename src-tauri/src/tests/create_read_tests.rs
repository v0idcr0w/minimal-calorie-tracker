use sqlx::SqlitePool;
use sqlx::types::chrono;
use crate::models::{food::Food, food_normalized::FoodNormalized, meal::Meal, daily_log::DailyLog}; 
use crate::utils::calc; 

// run tests with "cargo test -- --nocapture" to display the output of successful tests 

#[sqlx::test(fixtures("init_tables", "prepopulate"))]
async fn test_create_food(pool: SqlitePool) -> sqlx::Result<()> {

    let two_oranges = FoodNormalized::get_by_id(2, &pool).await?; 

    let mut two_oranges = Food::from(two_oranges, 280.0); 

    two_oranges.create_entry(&pool).await?; 

    let oranges_from_db = Food::get_by_id(two_oranges.id, &pool).await?;

    assert_eq!(two_oranges, oranges_from_db); 

    Ok(())
}

#[sqlx::test(fixtures("init_tables", "prepopulate"))]
async fn test_create_food_normalized(pool: SqlitePool) -> sqlx::Result<()> {
    let mut chicken = FoodNormalized::new(0, "chicken breast".to_string(), 120.0, "g".to_string(), 37.0, 0.0, 4.3, 198.0); 

    chicken.create_entry(&pool).await?; 

    let chicken_from_db = FoodNormalized::get_by_id(chicken.id, &pool).await?; 

    assert_eq!(chicken, chicken_from_db); 

    Ok(())
}

#[sqlx::test(fixtures("init_tables", "prepopulate", "foods"))]
async fn test_create_meal(pool: SqlitePool) -> sqlx::Result<()> {
    let mut breakfast = Meal::new(0, "breakfast".to_string()); 

    let food_ids = [1, 2]; 

    breakfast.create_entry(&pool, &food_ids).await?; 

    let foods_from_meal_id = Meal::get_foods_by_id(breakfast.id, &pool).await.unwrap(); 

    assert_eq!(foods_from_meal_id[0].id, food_ids[0]); 
    assert_eq!(foods_from_meal_id[1].id, food_ids[1]); 

    Ok(())

}

 
#[sqlx::test(fixtures("init_tables", "prepopulate", "foods", "meals"))]
async fn test_create_daily_log(pool: SqlitePool) -> sqlx::Result<()> {
    let total_macros = calc::compute_daily_totals(&[1, 2], &pool).await;

    let now = chrono::Local::now().naive_local(); 

    let mut daily_log = DailyLog::new(0, 59.6, total_macros.protein, total_macros.carbohydrate, total_macros.fat, total_macros.calories, now); 

    daily_log.create_entry(&pool).await?; 

    let daily_log_from_db = DailyLog::get_by_id(daily_log.id, &pool).await?; 

    assert_eq!(daily_log, daily_log_from_db); 

    Ok(())
}