use sqlx::{SqlitePool, query_as};
use super::super::models::{food::Food, food_normalized::FoodNormalized, meal::Meal, daily_log::DailyLog}; 


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
}

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


impl Meal {
    pub async fn get_foods_by_id(pk: i32, pool: &SqlitePool) -> Result<Vec<Food>, sqlx::Error> {
        // Gets all of the foods associated to a given meal_id. 
        let foods = query_as::<_, Food>("SELECT foods.* FROM foods JOIN meals_foods ON foods.id = meals_foods.food_id WHERE meals_foods.meal_id = ?")
        .bind(pk)
        .fetch_all(pool)
        .await?;
        
        Ok(foods)
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
}


