// use sqlx::SqlitePool;
// use sqlx::types::chrono;
// use crate::models::{food::Food, food_normalized::FoodNormalized, meal::Meal, daily_log::DailyLog}; 
// use crate::utils::calc;  

// #[sqlx::test(fixtures("init", "populate_normalized"))]
// async fn test_edit_food_normalized(pool: SqlitePool) -> sqlx::Result<()> {
//     let mut query = FoodNormalized::get_by_id(2, &pool).await?; 

//     let banana = FoodNormalized::new(2, "banana".to_string(), 118.0, "g".to_string(), 1.3, 27.0, 0.4, 105.0); 

//     query.update(banana.clone()); 

//     query.update_entry(&pool).await?; 

//     let query_from_db = FoodNormalized::get_by_id(2, &pool).await?; 

//     assert_eq!(banana, query_from_db); 

//     Ok(())
// }

// #[sqlx::test(fixtures("init", "populate_normalized", "populate_meals", "populate_foods"))]
// async fn test_edit_food(pool: SqlitePool) -> sqlx::Result<()> {
//     let mut food = Food::get_by_id(1, &pool).await?;  

//     food.update(100.0);

//     food.update_entry(&pool).await?; 

//     let food_from_db = Food::get_by_id(1, &pool).await?; 

//     assert_eq!(food, food_from_db);  

//     Ok(())
// }

// #[sqlx::test(fixtures("init", "populate_normalized", "populate_meals", "populate_foods"))]
// async fn test_edit_meal(pool: SqlitePool) -> sqlx::Result<()> {
//     let mut meal = Meal::get_by_id(1, &pool).await?; 
//     meal.update_name("lunch".to_string()); 
//     meal.update_entry(&pool).await?; 
//     let meal_from_db = Meal::get_by_id(1, &pool).await?; 
//     assert_eq!(meal, meal_from_db); 
//     Ok(())
// }

// #[sqlx::test(fixtures("init", "populate_normalized", "populate_meals", "populate_foods"))]
// async fn test_edit_daily_log_weight(pool: SqlitePool) -> sqlx::Result<()> {

//     let mut log = DailyLog::get_by_id(1, &pool).await?; 
//     log.weight = 55.5; // change weight
//     log.update_weight(&pool).await?; 

//     let log_from_db = DailyLog::get_by_id(1, &pool).await?; 

//     assert_eq!(log, log_from_db); 

//     Ok(())
// }

// #[sqlx::test(fixtures("init", "populate_normalized", "populate_meals", "populate_foods"))]
// async fn test_edit_daily_log_calories(pool: SqlitePool) -> sqlx::Result<()> {
//     let total_macros = calc::compute_daily_totals(&[1, 2], &pool).await;

//     let mut daily_log = DailyLog::get_by_id(1, &pool).await?;

//     daily_log.update_macros(total_macros); 
//     daily_log.update_entry(&pool).await?; 

//     let log_from_db = DailyLog::get_by_id(1, &pool).await?; 

//     assert_eq!(daily_log, log_from_db); 

//     Ok(())
// }