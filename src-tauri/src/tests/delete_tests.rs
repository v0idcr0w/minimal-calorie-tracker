use sqlx::SqlitePool;
use sqlx::types::chrono;
use crate::models::{food::Food, food_normalized::FoodNormalized, meal::Meal, daily_log::DailyLog}; 
use crate::utils::calc; 

#[sqlx::test(fixtures("init_tables", "prepopulate"))] 
async fn test_delete_food_normalized(pool: SqlitePool) -> sqlx::Result<()> {

    let food_normalized = FoodNormalized::get_by_id(1, &pool).await?; 
    food_normalized.delete_entry(&pool).await?; 

    let foods_remaining = FoodNormalized::get_all(&pool).await?; 

    assert_eq!(foods_remaining.len(), 2);  

    Ok(())
}
#[sqlx::test(fixtures("init_tables", "prepopulate", "foods"))] 
async fn test_delete_food_normalized_after_foods(pool: SqlitePool) -> sqlx::Result<()> {

    let food_normalized = FoodNormalized::get_by_id(1, &pool).await?; 
    food_normalized.delete_entry(&pool).await?; 

    let foods_remaining = FoodNormalized::get_all(&pool).await?; 

    assert_eq!(foods_remaining.len(), 2);  

    Ok(())
}


#[sqlx::test(fixtures("init_tables", "prepopulate", "foods"))] 
async fn test_delete_food(pool: SqlitePool) -> sqlx::Result<()> {

    let food = Food::get_by_id(1, &pool).await?; 
    food.delete_entry(&pool).await?; 

    let foods_remaining = Food::get_all(&pool).await?; 

    assert_eq!(foods_remaining.len(), 2);  

    Ok(())
}

#[sqlx::test(fixtures("init_tables", "prepopulate", "foods", "meals"))] 
async fn test_delete_food_after_meals(pool: SqlitePool) -> sqlx::Result<()> {

    let food = Food::get_by_id(1, &pool).await?; 

    food.delete_entry(&pool).await?; 

    let foods_remaining = Food::get_all(&pool).await?; 

    assert_eq!(foods_remaining.len(), 2);  

    Ok(())
}

#[sqlx::test(fixtures("init_tables", "prepopulate", "foods", "meals"))] 
async fn test_delete_meal(pool: SqlitePool) -> sqlx::Result<()> {
    let meal = Meal::get_by_id(1, &pool).await?; 

    meal.delete_entry(&pool).await?; 

    let meals_remaining = Meal::get_all(&pool).await?; 
    println!("{:?}", meals_remaining); 

    assert_eq!(meals_remaining.len(), 1);  

    Ok(())
}

// TODO! create new test for daily log, and editing
#[sqlx::test(fixtures("init_tables", "prepopulate", "foods", "meals"))] 
async fn test_delete_daily_log(pool: SqlitePool) -> sqlx::Result<()> {
    // this is copied and pasted from the other test 
    // creating entry in db
    let total_macros = calc::compute_daily_totals(&[1, 2], &pool).await;

    let now = chrono::Local::now().naive_local(); 

    let mut daily_log = DailyLog::new(0, 59.6, total_macros.protein, total_macros.carbohydrate, total_macros.fat, total_macros.calories, now); 

    daily_log.create_entry(&pool).await?; 

    // deleting entry in db
    daily_log.delete_entry(&pool).await?; 

    let query_result = DailyLog::get_all(&pool).await?; 
    assert!(query_result.len() == 0);


    Ok(())
}