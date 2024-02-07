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
use crate::models::{food::Food, food_normalized::FoodNormalized, meal::Meal, daily_log::DailyLog, error::Error, macros_total::MacrosTotal, recipe::Recipe, ingredient::Ingredient}; 
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
      update_food_normalized_name,
      get_meals, 
      get_meals_by_log_id,
      get_foods_by_meal_id, 
      add_new_meal,
      delete_meal,
      add_new_food,
      add_new_food_from_recipe,
      delete_food,
      update_food,
      compute_meal_macros,
      compute_daily_macros,
      get_all_logs,
      get_todays_log,
      weight_in,
      update_log_totals,
      get_all_recipes,
      create_new_recipe,
      update_recipe_name,
      delete_recipe,
      add_ingredient_to_recipe,
      update_recipe_serving_size,
      get_ingredients_by_recipe_id, 
      update_ingredient_amount,
      delete_ingredient_from_recipe])
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
async fn update_food_normalized(food: FoodNormalized, pool: tauri::State<'_,Database>) -> Result<FoodNormalized, Error> {
  food.update_entry(&pool.0).await?; 
  Ok(food)
}

#[tauri::command]
async fn update_food_normalized_name(mut food: FoodNormalized, new_name: String, pool: tauri::State<'_,Database>) -> Result<(), Error> {
  food.update_name(new_name, &pool.0).await?;  
  Ok(())
}

#[tauri::command]
async fn get_meals(pool: tauri::State<'_,Database>) -> Result<Vec<Meal>, Error> {
  let result = Meal::get_all(&pool.0).await?; 
  Ok(result) 
}

#[tauri::command]
async fn get_meals_by_log_id(log_id: i32, pool: tauri::State<'_,Database>) -> Result<Vec<Meal>, Error> {
  let result = Meal::get_by_log_id(log_id, &pool.0).await?; 
  Ok(result) 
}

#[tauri::command]
async fn get_foods_by_meal_id(meal_id: i32, pool: tauri::State<'_,Database>) -> Result<Vec<Food>, Error> {
  let result = Food::get_by_meal_id(meal_id, &pool.0).await?; 
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
  let mut food = Food::from_food_normalized(food_normalized, meal_id, amount); 
  // add to db 
  food.create_entry(&pool.0).await?; 

  Ok(())
}

#[tauri::command]
async fn add_new_food_from_recipe(selected_id: i32, amount: f64, meal_id: i32, pool: tauri::State<'_,Database>) -> Result<(), Error> {
  // query db for recipe 
  let recipe = Recipe::get_by_id(selected_id, &pool.0).await?; 
  // create food obj
  let mut food = Food::from_recipe(recipe, meal_id, amount); 
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
async fn update_food(mut food: Food, new_amount: f64, pool: tauri::State<'_,Database>) -> Result<Food, Error> {
  food.update(new_amount); 
  food.update_entry(&pool.0).await?; 
  
  Ok(food)
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

#[tauri::command]
async fn create_new_recipe(name: String, serving_size: f64, unit: String, pool: tauri::State<'_,Database>) -> Result<(), Error> {
  // id, macros and calories not required to make a new empty recipe 
  let mut new_recipe = Recipe::new(0, name, serving_size, unit, 0.0, 0.0, 0.0, 0.0); 
  new_recipe.create_entry(&pool.0).await?; 
  Ok(())
}

#[tauri::command] 
async fn get_all_recipes(pool: tauri::State<'_,Database>) -> Result<Vec<Recipe>, Error> {
  let result = Recipe::get_all(&pool.0).await?; 
  Ok(result) 
}

#[tauri::command]
async fn update_recipe_name(recipe_id: i32, new_name: String, pool: tauri::State<'_,Database>) -> Result<(), Error> {
  let mut recipe = Recipe::get_by_id(recipe_id, &pool.0).await?; 
  recipe.update_name(new_name, &pool.0).await?; 
  Ok(())
}

#[tauri::command]
async fn delete_recipe(recipe_id: i32, pool: tauri::State<'_,Database>) -> Result<(), Error> {
  let recipe = Recipe::get_by_id(recipe_id, &pool.0).await?; 
  recipe.delete_entry(&pool.0).await?; 
  Ok(())
}

#[tauri::command]
async fn add_ingredient_to_recipe(food_normalized: FoodNormalized, recipe_id: i32, amount: f64, pool: tauri::State<'_,Database>) -> Result<Recipe, Error> {
  // receives the recipe id, food normalized id and amount to create a new ingredient based on food_id, then adds its macros to the recipe. returns the updated recipe. 
  let mut ingredient = Ingredient::from(food_normalized, recipe_id, amount); 
  ingredient.create_entry(&pool.0).await?;

  // update the macros of the recipe
  let macros_to_add = ingredient.into_macros_total();
  let mut recipe = Recipe::get_by_id(recipe_id, &pool.0).await?;
  recipe.update_macros(macros_to_add);
  recipe.update_entry(&pool.0).await?;

  Ok(recipe)
}

#[tauri::command] 
async fn update_recipe_serving_size(mut recipe: Recipe, new_serving_size: f64, new_unit: String, pool: tauri::State<'_,Database>) -> Result<Recipe, Error> { 
  recipe.update_serving_size(new_serving_size, new_unit); 
  recipe.update_entry(&pool.0).await?;

  Ok(recipe)
}

#[tauri::command]
async fn get_ingredients_by_recipe_id(recipe_id: i32, pool: tauri::State<'_,Database>) -> Result<Vec<Ingredient>, Error> {
  let result = Ingredient::get_by_recipe_id(recipe_id, &pool.0).await?; 
  Ok(result) 
}

#[tauri::command]
async fn update_ingredient_amount(mut ingredient: Ingredient, new_amount: f64, pool: tauri::State<'_,Database>) -> Result<Recipe, Error> {
  // update ingredient 
  let macros_before = ingredient.into_macros_total(); 
  ingredient.update(new_amount); 
  ingredient.update_entry(&pool.0).await?;

  // calculate the macro difference 
  let delta_macros = ingredient.into_macros_total() - macros_before;

  // update recipe  
  let mut recipe = Recipe::get_by_id(ingredient.recipe_id, &pool.0).await?;
  recipe.update_macros(delta_macros);
  recipe.update_entry(&pool.0).await?;

  Ok(recipe)
}

#[tauri::command]
async fn delete_ingredient_from_recipe(ingredient: Ingredient, pool: tauri::State<'_,Database>) -> Result<Recipe, Error> {
  // delete ingredient 
  ingredient.delete_entry(&pool.0).await?;

  // calculate the macro difference 
  let delta_macros = ingredient.into_macros_total() * (-1.0);

  // update recipe  
  let mut recipe = Recipe::get_by_id(ingredient.recipe_id, &pool.0).await?;
  recipe.update_macros(delta_macros);
  recipe.update_entry(&pool.0).await?;

  Ok(recipe)
}
