// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// to run the app in development:
// cargo tauri dev 

mod models; 
mod database; 
mod utils; 
mod tests; 

use sqlx::SqlitePool; 
use chrono::Local; 
use crate::models::{food::Food, food_normalized::FoodNormalized, meal::Meal, daily_log::DailyLog, error::Error, macros_total::MacrosTotal}; 
use crate::utils::{db, calc}; 

struct Database(SqlitePool); 

#[tokio::main(flavor="current_thread")] 
async fn main() -> Result<(), sqlx::Error> {

  let pool = SqlitePool::connect(db::DB_URL).await?; 

  tauri::Builder::default()
    .manage(Database(pool)) 
    .invoke_handler(tauri::generate_handler![
      get_foods_normalized, 
      add_new_food_normalized,
      delete_food_normalized, 
      update_food_normalized,
      get_meals, 
      get_foods_by_meal_id, 
      add_new_meal,
      delete_meal,
      add_new_food,
      delete_food,
      update_food,
      compute_meal_macros,
      compute_daily_macros,
      get_all_logs,
      get_todays_log,
      weight_in,
      update_log_totals])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

  Ok(())
}

#[tauri::command]
async fn get_foods_normalized(pool: tauri::State<'_,Database>) -> Result<Vec<FoodNormalized>, Error> {
  let result = FoodNormalized::get_all(&pool.0).await?; 

  Ok(result) 
}

#[tauri::command]
async fn add_new_food_normalized(mut new_food: FoodNormalized, pool: tauri::State<'_,Database>) -> Result<(), Error> {
  new_food.create_entry(&pool.0).await?; 

  Ok(())
}

#[tauri::command]
async fn delete_food_normalized(food: FoodNormalized, pool: tauri::State<'_,Database>) -> Result<(), Error> {
  food.delete_entry(&pool.0).await?;

  Ok(())
}

#[tauri::command]
async fn update_food_normalized(food: FoodNormalized, pool: tauri::State<'_,Database>) -> Result<(), Error> {
  food.update_entry(&pool.0).await?; 

  Ok(())
}

#[tauri::command]
async fn get_meals(pool: tauri::State<'_,Database>) -> Result<Vec<Meal>, Error> {
  let result = Meal::get_all(&pool.0).await?; 
  Ok(result) 
}

#[tauri::command]
async fn get_foods_by_meal_id(meal_id: i32, pool: tauri::State<'_,Database>) -> Result<Vec<Food>, Error> {
  let result = Meal::get_foods_by_id(meal_id, &pool.0).await?; 
  Ok(result) 
}


#[tauri::command]
async fn add_new_meal(mut new_meal: Meal, pool: tauri::State<'_,Database>) -> Result<(), Error> {
  new_meal.create_entry(&pool.0).await?; 
  Ok(())
}

#[tauri::command]
async fn delete_meal(meal: Meal, pool: tauri::State<'_,Database>) -> Result<(), Error> {
  meal.delete_entry(&pool.0).await?; 
  Ok(())
}

#[tauri::command]
async fn add_new_food(selected_id: i32, amount: f64, meal_id: i32, pool: tauri::State<'_,Database>) -> Result<(), Error> {
  // query db for food normalized with the selected id
  let food_normalized = FoodNormalized::get_by_id(selected_id, &pool.0).await?; 
  // create food obj
  let mut food = Food::from(food_normalized, meal_id, amount); 
  // add to db 
  food.create_entry(&pool.0).await?; 

  Ok(())
}

#[tauri::command]
async fn delete_food(food: Food, pool: tauri::State<'_,Database>) -> Result<(), Error> {
  
  food.delete_entry(&pool.0).await?; 

  Ok(())
}

#[tauri::command]
async fn update_food(mut food: Food, new_amount: f64, pool: tauri::State<'_,Database>) -> Result<(), Error> {
  food.update(new_amount); 
  food.update_entry(&pool.0).await?; 

  Ok(())

}

#[tauri::command]
async fn compute_meal_macros(meal_id: i32, pool: tauri::State<'_,Database>) -> Result<MacrosTotal, Error> {
  let macros_total = calc::compute_meal_total(meal_id, &pool.0).await; 

  Ok(macros_total)
}

#[tauri::command]
async fn compute_daily_macros(meal_ids: Vec<i32>, pool: tauri::State<'_,Database>) -> Result<MacrosTotal, Error> {
  let macros_total = calc::compute_daily_totals(&meal_ids, &pool.0).await; 
  Ok(macros_total)
}

#[tauri::command]
async fn get_all_logs(pool: tauri::State<'_,Database>) -> Result<Vec<DailyLog>, Error> {
  let result = DailyLog::get_all(&pool.0).await?; 
  Ok(result) 
}

#[tauri::command]
async fn get_todays_log(pool: tauri::State<'_, Database>) -> Result<DailyLog, Error> {
  let today = Local::now().date_naive();  
  let query = DailyLog::get_by_date(today, &pool.0).await;
  match query {
    Ok(log) => Ok(log), 
    Err(_) => {
      let mut new_log = DailyLog::new(today); 
      new_log.create_entry(&pool.0).await?;
      Ok(new_log)
    }
  } 
}

#[tauri::command]
async fn weight_in(log_id: i32, weight: f64, pool: tauri::State<'_,Database>) -> Result<(), Error> {
  let mut log = DailyLog::get_by_id(log_id, &pool.0).await?; 
  log.weight = weight; 
  log.update_weight(&pool.0).await?;

  Ok(())
}

#[tauri::command]
async fn update_log_totals(log_id: i32, daily_totals: MacrosTotal, pool: tauri::State<'_,Database>) -> Result<(), Error> {
  let mut log = DailyLog::get_by_id(log_id, &pool.0).await?; 
  log.update_macros(daily_totals); 
  log.update_entry(&pool.0).await?;

  Ok(())
}