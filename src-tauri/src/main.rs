// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// to run the app in development:
// cargo tauri dev 

// to build:
// DATABASE_URL=sqlite://database.db cargo tauri build

mod models; 
mod database; 
mod utils; 
mod tests; 

use sqlx::SqlitePool; 
use chrono::Local; 
use crate::models::{food::Food, food_normalized::FoodNormalized, meal::Meal, daily_log::DailyLog, error::Error, macros_total::MacrosTotal, recipe::Recipe, ingredient::Ingredient, user_goal::UserGoal}; 
use crate::utils::{db, calc, file_ops}; 

struct Database(SqlitePool); 

#[tokio::main(flavor="current_thread")] 
async fn main() -> Result<(), sqlx::Error> {
  // initialization script is embedded in the binary 
  let init_sql_script = include_str!("../init.sql"); 
  // create database if it doesn't exist
  db::create(init_sql_script).await?; 
  // todo! check if the user is using an older version of the database that doesn't have the new column called is_constant on meals, and if so, run the sql script 

  let pool = SqlitePool::connect(db::DB_URL).await?; 

  tauri::Builder::default()
    .manage(Database(pool)) 
    .invoke_handler(tauri::generate_handler![
      get_foods_normalized, 
      add_new_food_normalized,
      delete_food_normalized, 
      update_food_normalized,
      update_food_normalized_name,
      get_all_meals, 
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
      delete_ingredient_from_recipe, 
      get_user_goal, 
      update_weight_goal,
      update_calories_goal, 
      update_macros_goal,
      write_foods_file, 
      read_foods_file,
      get_constant_meals,
      update_meal_status,
      update_meal_name,
      update_log_standalone,
      delete_log,
      update_meal_is_disabled])
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
async fn add_new_food_normalized(new_food: FoodNormalized, pool: tauri::State<'_,Database>) -> Result<(), Error> {
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
  let updated_food = food.update_entry(&pool.0).await?; 
  Ok(updated_food)
}

#[tauri::command]
async fn update_food_normalized_name(food: FoodNormalized, new_name: String, pool: tauri::State<'_,Database>) -> Result<(), Error> {
  food.update_name(new_name, &pool.0).await?;  
  Ok(())
}

#[tauri::command]
async fn get_all_meals(pool: tauri::State<'_,Database>) -> Result<Vec<Meal>, Error> {
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
async fn add_new_meal(new_meal: Meal, pool: tauri::State<'_,Database>) -> Result<(), Error> {
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
  // create food obj and add to db
  Food::from_food_normalized(food_normalized, meal_id, amount).create_entry(&pool.0).await?; 

  Ok(())
}

#[tauri::command]
async fn add_new_food_from_recipe(selected_id: i32, amount: f64, meal_id: i32, pool: tauri::State<'_,Database>) -> Result<(), Error> {
  // query db for recipe 
  let recipe = Recipe::get_by_id(selected_id, &pool.0).await?; 
  // create food obj and add to db 
  Food::from_recipe(recipe, meal_id, amount).create_entry(&pool.0).await?; 

  Ok(())
}

#[tauri::command]
async fn delete_food(food: Food, pool: tauri::State<'_,Database>) -> Result<(), Error> {
  
  food.delete_entry(&pool.0).await?; 

  Ok(())
}

#[tauri::command]
async fn update_food(food: Food, new_amount: f64, pool: tauri::State<'_,Database>) -> Result<Food, Error> {
  let updated_food = food.update(new_amount).update_entry(&pool.0).await?; 
  
  Ok(updated_food)
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
      let new_log = DailyLog::new(today).create_entry(&pool.0).await?;
      Ok(new_log)
    }
  } 
}

#[tauri::command]
async fn weight_in(log_id: i32, weight: f64, pool: tauri::State<'_,Database>) -> Result<(), Error> {
  DailyLog::get_by_id(log_id, &pool.0).await?.update_weight(weight, &pool.0).await?;

  Ok(())
}

#[tauri::command]
async fn update_log_totals(log_id: i32, daily_totals: MacrosTotal, pool: tauri::State<'_,Database>) -> Result<(), Error> {
  DailyLog::get_by_id(log_id, &pool.0).await?.update_macros(daily_totals).update_entry(&pool.0).await?;

  Ok(())
}

#[tauri::command]
async fn create_new_recipe(name: String, serving_size: f64, unit: String, pool: tauri::State<'_,Database>) -> Result<(), Error> {
  // id, macros and calories not required to make a new empty recipe 
  Recipe::new(0, name, serving_size, unit, 0.0, 0.0, 0.0, 0.0).create_entry(&pool.0).await?; 
  Ok(())
}

#[tauri::command] 
async fn get_all_recipes(pool: tauri::State<'_,Database>) -> Result<Vec<Recipe>, Error> {
  let result = Recipe::get_all(&pool.0).await?; 
  Ok(result) 
}

#[tauri::command]
async fn update_recipe_name(recipe_id: i32, new_name: String, pool: tauri::State<'_,Database>) -> Result<(), Error> {
  Recipe::get_by_id(recipe_id, &pool.0).await?.update_name(new_name, &pool.0).await?; 
  Ok(())
}

#[tauri::command]
async fn delete_recipe(recipe_id: i32, pool: tauri::State<'_,Database>) -> Result<(), Error> {
  Recipe::get_by_id(recipe_id, &pool.0).await?.delete_entry(&pool.0).await?; 
  Ok(())
}

#[tauri::command]
async fn add_ingredient_to_recipe(food_normalized: FoodNormalized, recipe_id: i32, amount: f64, pool: tauri::State<'_,Database>) -> Result<Recipe, Error> {
  // receives the recipe id, food normalized id and amount to create a new ingredient based on food_id, then adds its macros to the recipe. returns the updated recipe. 
  let ingredient = Ingredient::from(food_normalized, recipe_id, amount).create_entry(&pool.0).await?;

  // update the macros of the recipe
  let macros_to_add = ingredient.into_macros_total();
  let recipe = Recipe::get_by_id(recipe_id, &pool.0).await?.update_macros(macros_to_add).update_entry(&pool.0).await?;

  Ok(recipe)
}

#[tauri::command] 
async fn update_recipe_serving_size(recipe: Recipe, new_serving_size: f64, new_unit: String, pool: tauri::State<'_,Database>) -> Result<Recipe, Error> { 
  let updated_recipe = recipe.update_serving_size(new_serving_size, new_unit).update_entry(&pool.0).await?;

  Ok(updated_recipe)
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
  ingredient = ingredient.update(new_amount).update_entry(&pool.0).await?;

  // calculate the macro difference 
  let delta_macros = ingredient.into_macros_total() - macros_before;

  // update recipe  
  let recipe = Recipe::get_by_id(ingredient.recipe_id, &pool.0).await?.update_macros(delta_macros).update_entry(&pool.0).await?;

  Ok(recipe)
}

#[tauri::command]
async fn delete_ingredient_from_recipe(ingredient: Ingredient, pool: tauri::State<'_,Database>) -> Result<Recipe, Error> {
  // delete ingredient 
  ingredient.delete_entry(&pool.0).await?;

  // calculate the macro difference 
  let delta_macros = ingredient.into_macros_total() * (-1.0);

  // update recipe  
  let recipe = Recipe::get_by_id(ingredient.recipe_id, &pool.0).await?.update_macros(delta_macros).update_entry(&pool.0).await?;

  Ok(recipe)
}

#[tauri::command]
async fn get_user_goal(pool: tauri::State<'_,Database>) -> Result<UserGoal, Error> {
  let result = UserGoal::get_by_id(1, &pool.0).await; 
  match result {
    Ok(goal) => Ok(goal), 
    Err(_) => {
      let new_goal = UserGoal::new(0.0, 0.0, 0.0, 0.0, 2000.0).create_entry(&pool.0).await?;
      Ok(new_goal)
    }
  } 
}

#[tauri::command]
async fn update_weight_goal(new_user_goal: UserGoal, pool: tauri::State<'_,Database>) -> Result<UserGoal, Error> {
  new_user_goal.update_weight(&pool.0).await?;
  Ok(new_user_goal)
}

#[tauri::command]
async fn update_calories_goal(new_user_goal: UserGoal, pool: tauri::State<'_,Database>) -> Result<UserGoal, Error> {
  new_user_goal.update_calories(&pool.0).await?;
  Ok(new_user_goal)
}

#[tauri::command]
async fn update_macros_goal(new_user_goal: UserGoal, pool: tauri::State<'_,Database>) -> Result<UserGoal, Error> {
  new_user_goal.update_macros(&pool.0).await?;
  Ok(new_user_goal)
}

#[tauri::command]
async fn write_foods_file(file_path: String, pool: tauri::State<'_, Database>) -> Result<(),  Error> {
  let content = FoodNormalized::get_all(&pool.0).await?; 
  file_ops::write_csv_file(file_path, content).await?;
  Ok(())
}

#[tauri::command]
async fn read_foods_file(file_path: String, pool: tauri::State<'_, Database>) -> Result<(),  Error> {
  let content = file_ops::read_csv_file(file_path).await?;
  for food in content {
    food.create_entry(&pool.0).await?;
  }
  Ok(())
}

#[tauri::command]
async fn get_constant_meals(log_id: i32, pool: tauri::State<'_, Database>) -> Result<Vec<Meal>, Error> {
  let result = Meal::get_constant(log_id, &pool.0).await?; 
  Ok(result)
}

#[tauri::command]
async fn update_meal_status(meal: Meal, pool: tauri::State<'_, Database>) -> Result<(), Error> {
  meal.update_status(&pool.0).await?;
  Ok(())
}

#[tauri::command]
async fn update_meal_name(meal: Meal, new_name: String, pool: tauri::State<'_, Database>) -> Result<(), Error> {
  meal.update_name(new_name).update_entry(&pool.0).await?;
  Ok(())
}

#[tauri::command]
async fn update_log_standalone(log: DailyLog, pool: tauri::State<'_, Database>) -> Result<DailyLog, Error> {
  let updated_log = log.update_entry(&pool.0).await?;
  Ok(updated_log)
}

#[tauri::command]
async fn delete_log(log: DailyLog, pool: tauri::State<'_, Database>) -> Result<(), Error> {
  log.delete_entry(&pool.0).await?;
  Ok(())
}

#[tauri::command]
async fn update_meal_is_disabled(meal: Meal, pool: tauri::State<'_, Database>) -> Result<(), Error> {
  meal.update_is_disabled(&pool.0).await?;
  Ok(())
}