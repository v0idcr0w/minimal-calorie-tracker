use sqlx::{SqlitePool, query_as};
use chrono::{NaiveDate, NaiveDateTime}; 
use super::super::models::{food::Food, food_normalized::FoodNormalized, meal::Meal, daily_log::DailyLog, ingredient::Ingredient, recipe::Recipe}; 

impl FoodNormalized {
    pub async fn get_by_id(pk: i32, pool: &SqlitePool) -> Result<Self, sqlx::Error> {
        let food = query_as::<_, Self>("SELECT * FROM foods_normalized WHERE id = ?")
        .bind(pk)
        .fetch_one(pool)
        .await?;
        Ok(food)
    }

    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        let food_list = query_as::<_, Self>("SELECT * FROM foods_normalized")
        .fetch_all(pool)
        .await?;
        Ok(food_list)
    }    
}

impl Food {
    pub async fn get_by_id(pk: i32, pool: &SqlitePool) -> Result<Self, sqlx::Error> {
        let food = query_as::<_, Self>("SELECT * FROM foods WHERE id = ?")
        .bind(pk)
        .fetch_one(pool)
        .await?;
        Ok(food)
    }

    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        let food_list = query_as::<_, Self>("SELECT * FROM foods")
        .fetch_all(pool)
        .await?;
        Ok(food_list)
    }

    pub async fn get_by_meal_id(meal_id: i32, pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        let foods = query_as::<_, Self>("SELECT * FROM foods WHERE meal_id = ?")
        .bind(meal_id)
        .fetch_all(pool)
        .await?;
        Ok(foods)
    }

}




impl Meal {

    pub async fn get_by_log_id(log_id: i32, pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        let meals = query_as::<_, Self>("SELECT * FROM meals WHERE log_id = ?")
        .bind(log_id)
        .fetch_all(pool)
        .await?;
        Ok(meals)
    }

    pub async fn get_by_id(pk: i32, pool: &SqlitePool) ->  Result<Self, sqlx::Error> {
        let meal = query_as::<_, Meal>("SELECT * FROM meals WHERE id = ?")
        .bind(pk)
        .fetch_one(pool)
        .await?; 

        Ok(meal)
    }

    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        let meal_list = query_as::<_, Self>("SELECT * FROM meals")
        .fetch_all(pool)
        .await?;
        Ok(meal_list)
    }

    pub async fn get_by_date(date: NaiveDate, pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        let meals = query_as::<_, Self>("SELECT * FROM meals WHERE date(entry_timestamp) = ?")
        .bind(date)
        .fetch_all(pool)
        .await?;
        Ok(meals)
    }
}

impl DailyLog {
    pub async fn get_by_id(pk: i32, pool: &SqlitePool) -> Result<Self, sqlx::Error> {
        let logs = query_as::<_, Self>("SELECT * FROM daily_logs WHERE id = ?")
        .bind(pk)
        .fetch_one(pool)
        .await?;
    Ok(logs)
    }
    
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        let logs_list = query_as::<_, Self>("SELECT * FROM daily_logs")
        .fetch_all(pool)
        .await?;
        Ok(logs_list)
    }

    pub async fn get_by_date(date: NaiveDate, pool: &SqlitePool) -> Result<Self, sqlx::Error> {
        let log = query_as::<_, Self>("SELECT * FROM daily_logs WHERE entry_date = ?")
        .bind(date)
        .fetch_one(pool)
        .await?;
        Ok(log) 
    }
}

impl Recipe {
    pub async fn get_by_id(pk: i32, pool: &SqlitePool) -> Result<Self, sqlx::Error> {
        let recipe = query_as::<_, Self>("SELECT * FROM recipes WHERE id = ?")
        .bind(pk)
        .fetch_one(pool)
        .await?;
        Ok(recipe)
    }

    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        let recipe_list = query_as::<_, Self>("SELECT * FROM recipes")
        .fetch_all(pool)
        .await?;
        Ok(recipe_list)
    }
}

impl Ingredient {
    pub async fn get_by_id(pk: i32, pool: &SqlitePool) -> Result<Self, sqlx::Error> {
        let ingredient = query_as::<_, Self>("SELECT * FROM ingredients WHERE id = ?")
        .bind(pk)
        .fetch_one(pool)
        .await?;
        Ok(ingredient)
    }

    pub async fn get_by_recipe_id(recipe_id: i32, pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        let ingredients = query_as::<_, Self>("SELECT * FROM ingredients WHERE recipe_id = ?")
        .bind(recipe_id)
        .fetch_all(pool)
        .await?;
        Ok(ingredients)
    }

    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        let ingredient_list = query_as::<_, Self>("SELECT * FROM ingredients")
        .fetch_all(pool)
        .await?;
        Ok(ingredient_list)
    }
}