// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// to run the app in development:
// cargo tauri dev 

// to build:
// DATABASE_URL=sqlite://database.db cargo tauri build

mod models; 
mod utils; 
mod tests; 

use sqlx::SqlitePool; 
use chrono::Local; 

use crate::models::{food::Food, food_normalized::FoodNormalized, meal::Meal, daily_log::{DailyLog, WeightUnits}, error::Error, macros_total::MacrosTotal, recipe::Recipe, ingredient::Ingredient, user_goal::UserGoal}; 
use crate::utils::{db, calc, file_ops}; 
use tauri::State; 

struct AppState {
  pool: SqlitePool 
}


#[tokio::main(flavor="current_thread")] 
async fn main() -> Result<(), sqlx::Error> {
  // initialization script is embedded in the binary 
  let init_sql_script = include_str!("../init.sql"); 
  // create database if it doesn't exist
  db::create(init_sql_script).await?; 
  // todo! check if the user is using an older version of the database that doesn't have the new column called is_constant on meals, and if so, run the sql script 

  let pool = SqlitePool::connect(db::DB_URL).await?; 

  tauri::Builder::default()
    .manage(AppState { pool }) 
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
      update_recipe,
      delete_recipe,
      add_ingredient_to_recipe,
      get_ingredients_by_recipe_id, 
      update_ingredient_amount,
      delete_ingredient_from_recipe, 
      get_user_goal, 
      update_user_goal,
      write_foods_file, 
      read_foods_file,
      get_constant_meals,
      update_meal_is_constant,
      update_meal_is_disabled,
      update_meal_name,
      update_log_standalone,
      delete_log,
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

  Ok(())
}

#[tauri::command]
async fn get_foods_normalized(state: State<'_, AppState>) -> Result<Vec<FoodNormalized>, Error> {
  let result = FoodNormalized::get_all(&state.pool).await?; 
  Ok(result) 
}

#[tauri::command]
async fn add_new_food_normalized(new_food: FoodNormalized, state: State<'_, AppState>) -> Result<FoodNormalized, Error> {
  let new_food = new_food.create(&state.pool).await?; 
  Ok(new_food)
}

#[tauri::command]
async fn delete_food_normalized(food: FoodNormalized, state: State<'_, AppState>) -> Result<(), Error> {
  food.delete(&state.pool).await?;
  Ok(())
}

#[tauri::command]
async fn update_food_normalized(food: FoodNormalized, state: State<'_, AppState>) -> Result<FoodNormalized, Error> {
  let updated_food = food.update(&state.pool).await?; 
  Ok(updated_food)
}

#[tauri::command]
async fn update_food_normalized_name(id: i32, new_name: String, state: State<'_, AppState>) -> Result<FoodNormalized, Error> {
  let mut tx = state.pool.begin().await?; 
  let food = FoodNormalized::get_by_id(id, &mut tx).await?.update_name(&new_name, &mut tx).await?;  

  tx.commit().await?;
  Ok(food)
}

#[tauri::command]
async fn get_all_meals(state: State<'_, AppState>) -> Result<Vec<Meal>, Error> {
  let result = Meal::get_all(&state.pool).await?; 
  Ok(result) 
}

#[tauri::command]
async fn get_meals_by_log_id(log_id: i32, state: State<'_, AppState>) -> Result<Vec<Meal>, Error> {
  let result = Meal::get_by_log_id(log_id, &state.pool).await?; 
  Ok(result) 
}

#[tauri::command]
async fn get_foods_by_meal_id(meal_id: i32, state: State<'_, AppState>) -> Result<Vec<Food>, Error> {
  let result = Food::get_by_meal_id(meal_id, &state.pool).await?; 
  Ok(result) 
}


#[tauri::command]
async fn add_new_meal(new_meal: Meal, state: State<'_, AppState>) -> Result<Meal, Error> {
  let new_meal = new_meal.create(&state.pool).await?; 
  Ok(new_meal)
}

#[tauri::command]
async fn delete_meal(meal: Meal, state: State<'_, AppState>) -> Result<(), Error> {
  meal.delete(&state.pool).await?; 
  Ok(())
}

#[tauri::command]
async fn add_new_food(selected_id: i32, amount: f64, meal_id: i32, state: State<'_, AppState>) -> Result<Food, Error> {
  // query db for food normalized with the selected id
  let mut tx = state.pool.begin().await?; 

  let food_normalized = FoodNormalized::get_by_id(selected_id, &mut tx).await?;
  // create food obj and add to db
  let food = Food::from_food_normalized(food_normalized, meal_id, amount, 0).create(&mut tx).await?;

  tx.commit().await?;
  Ok(food)
}

#[tauri::command]
async fn add_new_food_from_recipe(selected_id: i32, amount: f64, meal_id: i32, state: State<'_, AppState>) -> Result<Food, Error> {
  // query db for recipe 
  let mut tx = state.pool.begin().await?;  
  let recipe = sqlx::query_as("SELECT * FROM recipes WHERE id = ?")
    .bind(selected_id)
    .fetch_one(&mut *tx)
    .await?;

  // create food obj and add to db 
  let food = Food::from_recipe(recipe, meal_id, amount, 0);
  let food = sqlx::query_as("INSERT INTO foods (meal_id, food_normalized_id, name, amount, unit, protein, carbohydrate, fat, calories) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING *")
    .bind(food.meal_id)
    .bind(food.food_normalized_id)
    .bind(food.name)
    .bind(food.amount)
    .bind(food.unit)
    .bind(food.protein)
    .bind(food.carbohydrate)
    .bind(food.fat)
    .bind(food.calories)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
  Ok(food)
}

#[tauri::command]
async fn delete_food(food: Food, state: State<'_, AppState>) -> Result<(), Error> {
  
  food.delete(&state.pool).await?; 

  Ok(())
}

#[tauri::command]
async fn update_food(food: Food, new_amount: f64, state: State<'_, AppState>) -> Result<Food, Error> {
  // food may come either from the recipes or food normalized. 
  let mut tx = state.pool.begin().await?;
  let updated_food: Food = match (food.food_normalized_id, food.recipe_id) {
    (Some(id), None) => {
      let food_normalized: FoodNormalized = sqlx::query_as("SELECT * FROM foods_normalized WHERE id = ?")
      .bind(id)
      .fetch_one(&mut *tx)
      .await?;
      let food_from_normalized = Food::from_food_normalized(food_normalized, food.meal_id, new_amount, food.id);
      sqlx::query_as("UPDATE foods SET amount = ?, protein = ?, carbohydrate = ?, fat = ?, calories = ? WHERE id = ? RETURNING *")
      .bind(new_amount)
      .bind(food_from_normalized.protein)
      .bind(food_from_normalized.carbohydrate)
      .bind(food_from_normalized.fat)
      .bind(food_from_normalized.calories)
      .bind(food_from_normalized.id)
      .fetch_one(&mut *tx)
      .await?
    },
    (None, Some(id)) => {
      let recipe = sqlx::query_as("SELECT * FROM recipes WHERE id = ?")
      .bind(id)
      .fetch_one(&mut *tx)
      .await?;
      let food_from_recipe = Food::from_recipe(recipe, food.meal_id, new_amount, food.id);
      sqlx::query_as("UPDATE foods SET amount = ?, protein = ?, carbohydrate = ?, fat = ?, calories = ? WHERE id = ? RETURNING *")
      .bind(new_amount)
      .bind(food_from_recipe.protein)
      .bind(food_from_recipe.carbohydrate)
      .bind(food_from_recipe.fat)
      .bind(food_from_recipe.calories)
      .bind(food_from_recipe.id)
      .fetch_one(&mut *tx)
      .await?
    },
    _ => {
      Err(sqlx::Error::RowNotFound)?
    }
  }; 

  tx.commit().await?; 

  Ok(updated_food)
}

#[tauri::command]
async fn compute_meal_macros(meal_id: i32, state: State<'_, AppState>) -> Result<MacrosTotal, Error> {
  let foods = Food::get_by_meal_id(meal_id, &state.pool).await?;
  let macros_total = calc::compute_meal_total(foods); 

  Ok(macros_total)
}

#[tauri::command]
async fn compute_daily_macros(meal_ids: Vec<i32>, state: State<'_, AppState>) -> Result<MacrosTotal, Error> {
  let macros_total = calc::compute_daily_totals(&meal_ids, &state.pool).await; 
  Ok(macros_total)
}

#[tauri::command]
async fn get_all_logs(state: State<'_, AppState>) -> Result<Vec<DailyLog>, Error> {
  let result = DailyLog::get_all(&state.pool).await?; 
  Ok(result) 
}

#[tauri::command]
async fn get_todays_log(state: State<'_, AppState>) -> Result<DailyLog, Error> {
  let mut tx = state.pool.begin().await?; 
  
  let today = Local::now().date_naive();  
  let query = DailyLog::get_by_date(today, &mut tx).await;
  let result = match query {
    Ok(log) => Ok(log), 
    Err(_) => {
      let new_log = DailyLog::new(today, WeightUnits::Kg).create(&mut tx).await?;
      Ok(new_log)
    }
  }; 

  tx.commit().await?;
  result 
}

#[tauri::command]
async fn weight_in(log_id: i32, weight: f64, units: WeightUnits, state: State<'_, AppState>) -> Result<DailyLog, Error> {
  let mut tx = state.pool.begin().await?; 

  let updated_log = DailyLog::get_by_id(log_id, &mut tx).await?.update_weight(weight, units, &mut tx).await?;

  tx.commit().await?; 
  Ok(updated_log)
}

#[tauri::command]
async fn update_log_totals(log_id: i32, daily_totals: MacrosTotal, state: State<'_, AppState>) -> Result<DailyLog, Error> {
  let mut tx = state.pool.begin().await?; 

  let updated_log = DailyLog::get_by_id(log_id, &mut tx).await?.update_macros(daily_totals).update(&mut tx).await?;

  tx.commit().await?;
  Ok(updated_log)
}

#[tauri::command]
async fn create_new_recipe(name: String, serving_size: f64, unit: String, state: State<'_, AppState>) -> Result<Recipe, Error> {
  // id, macros and calories not required to make a new empty recipe 
  let new_recipe = Recipe::new(0, name, serving_size, unit, 0.0, 0.0, 0.0, 0.0).create(&state.pool).await?; 
  Ok(new_recipe)
}

#[tauri::command] 
async fn get_all_recipes(state: State<'_, AppState>) -> Result<Vec<Recipe>, Error> {
  let result = Recipe::get_all(&state.pool).await?; 
  Ok(result) 
}

#[tauri::command]
async fn update_recipe(recipe: Recipe, state: State<'_, AppState>) -> Result<Recipe, Error> {
  let mut tx = state.pool.begin().await?;
  let mut recipe_from_db = Recipe::get_by_id(recipe.id, &mut tx).await?.update_serving_size(recipe.serving_size, recipe.unit); 
  // update name if required
  recipe_from_db.name = recipe.name; 
  let updated_recipe = recipe_from_db.update(&mut tx).await?;
  tx.commit().await?;
  Ok(updated_recipe)
}

#[tauri::command]
async fn delete_recipe(recipe_id: i32, state: State<'_, AppState>) -> Result<(), Error> {
  let mut tx = state.pool.begin().await?;
  Recipe::get_by_id(recipe_id, &mut tx).await?.delete(&mut tx).await?; 
  tx.commit().await?; 
  Ok(())
}

#[tauri::command]
async fn add_ingredient_to_recipe(ingredient_list: Vec<FoodNormalized>, recipe_id: i32, state: State<'_, AppState>) -> Result<Vec<Ingredient>, Error> {
  // receives a list of ingredients as normalized food items and recipe id, then creates new ingriedients with a default amount of 0.0. returns the list of ingredients as inserted in the database. 
  let mut tx = state.pool.begin().await?;
  let mut ingredients_from_db: Vec<Ingredient> = vec![];  
  for ingredient in ingredient_list.into_iter() {
    let ingredient_from_db = Ingredient::from(ingredient, recipe_id, 0.0).create(&mut tx).await?;
    ingredients_from_db.push(ingredient_from_db);
  }

  tx.commit().await?;
  Ok(ingredients_from_db)
}

#[tauri::command]
async fn get_ingredients_by_recipe_id(recipe_id: i32, state: State<'_, AppState>) -> Result<Vec<Ingredient>, Error> {
  let result = Ingredient::get_by_recipe_id(recipe_id, &state.pool).await?; 
  Ok(result) 
}

#[tauri::command]
async fn update_ingredient_amount(mut ingredient: Ingredient, new_amount: f64, state: State<'_, AppState>) -> Result<Recipe, Error> {
  let mut tx = state.pool.begin().await?;
  // update ingredient 
  let macros_before = ingredient.into_macros_total(); 
  let ingredient_as_food = FoodNormalized::get_by_id(ingredient.food_normalized_id, &mut tx).await?; 
  ingredient = ingredient.update(new_amount, ingredient_as_food, &mut tx).await?;

  // calculate the macro difference 
  let delta_macros = ingredient.into_macros_total() - macros_before;

  // update recipe  
  let recipe = Recipe::get_by_id(ingredient.recipe_id, &mut tx).await?.update_macros(delta_macros).update(&mut tx).await?;

  tx.commit().await?; 
  Ok(recipe)
}

#[tauri::command]
async fn delete_ingredient_from_recipe(ingredient: Ingredient, state: State<'_, AppState>) -> Result<Recipe, Error> {
  let mut tx = state.pool.begin().await?;
  let recipe_id = ingredient.recipe_id; 
  // calculate the macro difference 
  let delta_macros = ingredient.into_macros_total() * (-1.0);
  // delete ingredient 
  ingredient.delete(&mut tx).await?;
  // update recipe  
  let recipe = Recipe::get_by_id(recipe_id, &mut tx).await?.update_macros(delta_macros).update(&mut tx).await?;

  tx.commit().await?; 
  Ok(recipe)
}

#[tauri::command]
async fn get_user_goal(state: State<'_, AppState>) -> Result<UserGoal, Error> {
  let mut tx = state.pool.begin().await?;
  let result = UserGoal::get_by_id(1, &mut tx).await; 
  let user_goal = match result {
    Ok(goal) => Ok(goal), 
    Err(_) => {
      let new_goal = UserGoal::new(0.0, 0.0, 0.0, 0.0, 2000.0).create(&mut tx).await?;
      Ok(new_goal)
    }
  };
  tx.commit().await?;
  user_goal
}

#[tauri::command]
async fn update_user_goal(new_user_goal: UserGoal, state: State<'_, AppState>) -> Result<UserGoal, Error> {
  let new_user_goal = new_user_goal.update(&state.pool).await?;
  Ok(new_user_goal)
}


#[tauri::command]
async fn write_foods_file(file_path: String, state: State<'_, AppState>) -> Result<(),  Error> {
  let content = FoodNormalized::get_all(&state.pool).await?; 
  file_ops::write_csv_file(file_path, content).await?;
  Ok(())
}

#[tauri::command]
async fn read_foods_file(file_path: String, state: State<'_, AppState>) -> Result<(),  Error> {
  let content = file_ops::read_csv_file(file_path).await?;
  for food in content {
    FoodNormalized::from(food).create(&state.pool).await?;
  }
  Ok(())
}

#[tauri::command]
async fn get_constant_meals(log_id: i32, state: State<'_, AppState>) -> Result<Vec<Meal>, Error> {
  let result = Meal::get_all_constant(log_id, &state.pool).await?; 
  Ok(result)
}

#[tauri::command]
async fn update_meal_is_constant(meal_id: i32, status: bool, state: State<'_, AppState>) -> Result<Meal, Error> {
  let mut tx = state.pool.begin().await?;
  let updated_meal = Meal::get_by_id(meal_id, &mut tx).await?.update_constant_status(status, &mut tx).await?;
  tx.commit().await?;
  Ok(updated_meal)
}

#[tauri::command]
async fn update_meal_is_disabled(meal_id: i32, status: bool, state: State<'_, AppState>) -> Result<Meal, Error> {
  let mut tx = state.pool.begin().await?;
  let updated_meal = Meal::get_by_id(meal_id, &mut tx).await?.update_disabled_status(status, &mut tx).await?;
  tx.commit().await?;
  Ok(updated_meal)
}

#[tauri::command]
async fn update_meal_name(meal_id: i32, new_name: String, state: State<'_, AppState>) -> Result<Meal, Error> {
  let mut tx = state.pool.begin().await?;
  let updated_meal = Meal::get_by_id(meal_id, &mut tx).await?.update_name(&new_name, &mut tx).await?;
  tx.commit().await?;
  Ok(updated_meal)
}

#[tauri::command]
async fn update_log_standalone(log: DailyLog, state: State<'_, AppState>) -> Result<DailyLog, Error> {
  let mut tx = state.pool.begin().await?;
  let updated_log = log.update(&mut tx).await?;
  tx.commit().await?;
  Ok(updated_log)
}

#[tauri::command]
async fn delete_log(log: DailyLog, state: State<'_, AppState>) -> Result<(), Error> {
  log.delete(&state.pool).await?;
  Ok(())
}

