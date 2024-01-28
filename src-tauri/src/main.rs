// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// to run the app in development:
// cargo tauri dev 

mod models; 
mod database; 
mod utils; 
mod tests; 

use sqlx::SqlitePool; 
use crate::models::{food::Food, food_normalized::FoodNormalized, meal::Meal, daily_log::DailyLog, error::Error}; 
use crate::database::{create_ops, read_ops, update_ops, delete_ops}; 
use crate::utils::{db, calc}; 

#[tokio::main(flavor="current_thread")] 
async fn main() -> Result<(), sqlx::Error> {


  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      get_foods_normalized, 
      add_new_food_normalized,
      delete_food_normalized, 
      update_food_normalized])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

  Ok(())
}

#[tauri::command]
async fn get_foods_normalized() -> Result<Vec<FoodNormalized>, Error> {
  let pool = SqlitePool::connect(db::DB_URL).await?; 
  let result = FoodNormalized::get_all(&pool).await?; 

  Ok(result) 
}

#[tauri::command]
async fn add_new_food_normalized(mut new_food: FoodNormalized) -> Result<(), Error> {
  let pool = SqlitePool::connect(db::DB_URL).await?;  
  new_food.create_entry(&pool).await?; 

  Ok(())
}

#[tauri::command]
async fn delete_food_normalized(food: FoodNormalized) -> Result<(), Error> {
  let pool = SqlitePool::connect(db::DB_URL).await?; 
  food.delete_entry(&pool).await?;

  Ok(())
}

#[tauri::command]
async fn update_food_normalized(food: FoodNormalized) -> Result<(), Error> {
  let pool = SqlitePool::connect(db::DB_URL).await?;  
  food.update_entry(&pool).await?; 

  Ok(())
}