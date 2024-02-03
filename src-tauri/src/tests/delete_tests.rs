use sqlx::SqlitePool;
use sqlx::types::chrono;
use crate::models::{food::Food, food_normalized::FoodNormalized, meal::Meal, daily_log::DailyLog}; 
use crate::utils::calc; 

#[sqlx::test(fixtures("init", "populate_normalized"))]
async fn test_delete_food_normalized(pool: SqlitePool) -> sqlx::Result<()> {

    let total_foods = FoodNormalized::get_all(&pool).await?.len(); 

    let food_normalized = FoodNormalized::get_by_id(1, &pool).await?; 
    food_normalized.delete_entry(&pool).await?; 

    let foods_remaining = FoodNormalized::get_all(&pool).await?; 

    assert_eq!(foods_remaining.len(), total_foods-1);  

    Ok(())
}
#[sqlx::test(fixtures("init", "populate_normalized", "populate_meals", "populate_foods"))]
async fn test_delete_food_normalized_after_foods(pool: SqlitePool) -> sqlx::Result<()> {
    let total_foods = FoodNormalized::get_all(&pool).await?.len(); 

    let food_normalized = FoodNormalized::get_by_id(1, &pool).await?; 
    food_normalized.delete_entry(&pool).await?; 

    let foods_remaining = FoodNormalized::get_all(&pool).await?; 

    assert_eq!(foods_remaining.len(), total_foods-1);  

    Ok(())
}


#[sqlx::test(fixtures("init", "populate_normalized", "populate_meals", "populate_foods"))]
async fn test_delete_food(pool: SqlitePool) -> sqlx::Result<()> {
    let total_foods = Food::get_all(&pool).await?.len();

    let food = Food::get_by_id(1, &pool).await?; 
    food.delete_entry(&pool).await?; 

    let foods_remaining = Food::get_all(&pool).await?; 

    assert_eq!(foods_remaining.len(), total_foods-1);  

    Ok(())
}

#[sqlx::test(fixtures("init", "populate_normalized", "populate_meals", "populate_foods"))]
async fn test_delete_food_after_meals(pool: SqlitePool) -> sqlx::Result<()> {
    let total_foods_of_meal = Meal::get_foods_by_id(1, &pool).await?.len(); 

    let food = Food::get_by_id(1, &pool).await?; 

    food.delete_entry(&pool).await?; 

    let foods_remaining = Meal::get_foods_by_id(1, &pool).await?; 

    assert_eq!(foods_remaining.len(), total_foods_of_meal-1);  

    Ok(())
}

#[sqlx::test(fixtures("init", "populate_normalized", "populate_meals", "populate_foods"))]
async fn test_delete_meal(pool: SqlitePool) -> sqlx::Result<()> {
    let total_meals = Meal::get_all(&pool).await?.len(); 

    let meal = Meal::get_by_id(1, &pool).await?; 

    meal.delete_entry(&pool).await?; 

    let meals_remaining = Meal::get_all(&pool).await?; 
    println!("{:?}", meals_remaining); 

    assert_eq!(meals_remaining.len(), total_meals-1);  

    Ok(())
}

#[sqlx::test(fixtures("init", "populate_normalized", "populate_meals", "populate_foods"))]
async fn test_delete_daily_log(pool: SqlitePool) -> sqlx::Result<()> {
    // this is copied and pasted from the other test 
    // creating entry in db
    let total_logs = DailyLog::get_all(&pool).await?.len();
    
    let now = chrono::Local::now().date_naive(); 

    let mut daily_log = DailyLog::new(now); 

    daily_log.create_entry(&pool).await?; 

    // deleting entry in db
    daily_log.delete_entry(&pool).await?; 

    let query_result = DailyLog::get_all(&pool).await?; 
    assert_eq!(query_result.len(), total_logs  );

    Ok(())
}