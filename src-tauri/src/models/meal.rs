use chrono::{NaiveDateTime, NaiveDate};
use sqlx::{FromRow, SqlitePool}; 
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, FromRow)] 
pub struct Meal {
    pub id: i32, 
    pub log_id: i32, 
    pub name: String, 
    pub entry_timestamp: NaiveDateTime, 
    pub is_constant: Option<bool>, 
    pub is_disabled: Option<bool>
}

impl Meal {
    pub fn new(id: i32, log_id: i32, name: String, entry_timestamp: NaiveDateTime, is_constant: Option<bool>, is_disabled: Option<bool>) -> Self {
        Self { id, log_id, name, entry_timestamp, is_constant, is_disabled }
    }
    pub async fn create(self, db: &SqlitePool) -> Result<Self, sqlx::Error> {
        let meal = sqlx::query_as("INSERT INTO meals (name, entry_timestamp, log_id, is_constant, is_disabled) VALUES (?, ?, ?, ?, ?) RETURNING *")
        .bind(self.name.to_lowercase())
        .bind(self.entry_timestamp)
        .bind(self.log_id)
        .bind(self.is_constant)
        .bind(self.is_disabled)
        .fetch_one(db)
        .await; 

        meal
    }
    pub async fn update_name(self, new_name: &str, db: &SqlitePool) -> Result<Self, sqlx::Error> {
        let updated_meal = sqlx::query_as("UPDATE meals SET name = ? WHERE id = ? RETURNING *")
        .bind(new_name)
        .bind(self.id)
        .fetch_one(db)
        .await; 
        updated_meal 
    }
    pub async fn update_constant_status(self, db: &SqlitePool) -> Result<Self, sqlx::Error> {
        let updated_meal = sqlx::query_as("UPDATE meals SET is_constant = ? WHERE id = ? RETURNING *")
        .bind(self.is_constant)
        .bind(self.id)
        .fetch_one(db)
        .await; 
        updated_meal 
    }
    pub async fn update_disabled_status(self, db: &SqlitePool) -> Result<Self, sqlx::Error> {
        let updated_meal = sqlx::query_as("UPDATE meals SET is_disabled = ? WHERE id = ? RETURNING *")
        .bind(self.is_disabled)
        .bind(self.id)
        .fetch_one(db)
        .await; 
        updated_meal 
    }
    pub async fn delete(self, db: &SqlitePool) -> Result<(), sqlx::Error> {
        let _ = sqlx::query("DELETE FROM meals WHERE id = ?").bind(self.id).execute(db).await?;
        Ok(())
    }
    pub async fn get_by_id(pk: i32, pool: &SqlitePool) ->  Result<Self, sqlx::Error> {
        let meal = sqlx::query_as::<_, Meal>("SELECT * FROM meals WHERE id = ?")
        .bind(pk)
        .fetch_one(pool)
        .await?; 

        Ok(meal)
    }
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        let meal_list = sqlx::query_as::<_, Self>("SELECT * FROM meals")
        .fetch_all(pool)
        .await?;
        Ok(meal_list)
    }
    pub async fn get_by_log_id(log_id: i32, pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        let meals = sqlx::query_as::<_, Self>("SELECT * FROM meals WHERE log_id = ?")
        .bind(log_id)
        .fetch_all(pool)
        .await?;
        Ok(meals)
    }
    pub async fn get_all_constant(log_id: i32, pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        // this method retrieves all meals marked as constant with the exception of those that have the same log_id as the current log_id. 
        let meals = sqlx::query_as::<_, Self>("SELECT * FROM meals WHERE is_constant = 1 AND log_id != ?")
        .bind(log_id)
        .fetch_all(pool)
        .await?;
        Ok(meals)
    }
    pub async fn get_by_date(date: NaiveDate, pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        let meals = sqlx::query_as::<_, Self>("SELECT * FROM meals WHERE date(entry_timestamp) = ?")
        .bind(date)
        .fetch_all(pool)
        .await?;
        Ok(meals)
    }
}