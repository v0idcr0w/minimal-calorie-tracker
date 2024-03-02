// use ::chrono::Duration;
// use sqlx::SqlitePool;
// use sqlx::types::chrono;
// use crate::models::{food::Food, food_normalized::FoodNormalized, meal::Meal, daily_log::DailyLog}; 
// use crate::utils::calc; 

// // run tests with "cargo test -- --nocapture" to display the output of successful tests 

// #[sqlx::test(fixtures("init", "populate_normalized"))]
// async fn test_create_food_normalized(pool: SqlitePool) -> sqlx::Result<()> {
//     let mut chicken = FoodNormalized::new(0, "chicken breast".to_string(), 120.0, "g".to_string(), 37.0, 0.0, 4.3, 198.0); 

//     chicken.create_entry(&pool).await?; 

//     let chicken_from_db = FoodNormalized::get_by_id(chicken.id, &pool).await?; 

//     assert_eq!(chicken, chicken_from_db); 

//     Ok(())
// }


// #[sqlx::test(fixtures("init", "populate_normalized", "populate_meals"))]
// async fn test_create_food(pool: SqlitePool) -> sqlx::Result<()> {

//     let food_norm = FoodNormalized::get_by_id(2, &pool).await?; 

//     let mut food = Food::from_food_normalized(food_norm, 1, 280.0); 

//     food.create_entry(&pool).await?; 

//     let food_from_db = Food::get_by_id(food.id, &pool).await?;

//     assert_eq!(food, food_from_db); 

//     Ok(())
// }

// #[sqlx::test(fixtures("init", "populate_normalized", "populate_meals"))]
// async fn test_create_meal_no_foods(pool: SqlitePool) -> sqlx::Result<()> {
//     let today = chrono::Local::now().naive_local();
//     let mut brunch = Meal::new(0, 1, "BRUNCH".to_string(), today); 

//     brunch.create_entry(&pool).await?; 
//     let brunch_from_db = Meal::get_by_id(brunch.id, &pool).await?; 
//     println!("{:?}", brunch_from_db);

//     assert_eq!(brunch, brunch_from_db); 

//     Ok(())
// }

// #[sqlx::test(fixtures("init", "populate_normalized", "populate_meals", "populate_foods"))]
// async fn test_create_daily_log(pool: SqlitePool) -> sqlx::Result<()> {
//     let total_macros = calc::compute_daily_totals(&[1, 2], &pool).await;

//     let now = chrono::Local::now().date_naive(); 

//     let mut daily_log = DailyLog::new(now); 
//     daily_log.update_macros(total_macros);
//     daily_log.weight = 180.0;  
//     daily_log.update_weight(&pool).await?; 
//     daily_log.create_entry(&pool).await?; 

//     let daily_log_from_db = DailyLog::get_by_id(daily_log.id, &pool).await?; 

//     assert_eq!(daily_log, daily_log_from_db); 

//     Ok(())
// }


// #[sqlx::test(fixtures("init", "populate_normalized", "populate_meals", "populate_foods"))]
// async fn test_get_meals_by_date(pool: SqlitePool) -> sqlx::Result<()> {
//     let total_meals = Meal::get_all(&pool).await?.len();
//     let today = chrono::Local::now().date_naive(); 
//     let meals = Meal::get_by_date(today, &pool).await?; 

//     assert_eq!(meals.len(), total_meals); 

//     Ok(())

// }

// #[sqlx::test(fixtures("init", "populate_normalized", "populate_meals", "populate_foods"))]
// async fn test_get_daily_log_by_date(pool: SqlitePool) -> sqlx::Result<()> {
//     let tomorrow = chrono::Local::now().date_naive() + Duration::days(1); 
//     let mut daily_log = DailyLog::new(tomorrow); 
//     daily_log.create_entry(&pool).await?; 

//     let daily_log_from_db = DailyLog::get_by_date(tomorrow, &pool).await?; 
 
//     assert_eq!(daily_log, daily_log_from_db); 

//     Ok(())
// }

// #[sqlx::test(fixtures("init", "populate_normalized", "populate_meals", "populate_foods"))]
// async fn test_get_meals_by_log_id(pool: SqlitePool) -> sqlx::Result<()> {
//     let total_meals = Meal::get_all(&pool).await?.len(); 
//     let meals = Meal::get_by_log_id(1, &pool).await?; 

//     assert_eq!(meals.len(), total_meals); 
//     Ok(())
// }