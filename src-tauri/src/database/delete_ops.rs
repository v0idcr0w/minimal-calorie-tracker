use sqlx::SqlitePool;
use super::super::models::{food::Food, meal::Meal, food_normalized::FoodNormalized, daily_log::DailyLog, ingredient::Ingredient, recipe::Recipe}; 

impl FoodNormalized {
    pub async fn delete_entry(&self, pool: &SqlitePool) -> Result<(), sqlx::Error> {

        sqlx::query!("DELETE FROM foods_normalized WHERE id = ?", self.id)
        .execute(pool)
        .await?; 

        println!("[INFO] Deleted entry with id = {}", self.id); 
        Ok(())
    }
}

impl Food {
    pub async fn delete_entry(&self, pool: &SqlitePool) -> Result<(), sqlx::Error> {

        sqlx::query!("DELETE FROM foods WHERE id = ?", self.id)
        .execute(pool)
        .await?; 
    
        println!("[INFO] Deleted entry with id = {}", self.id); 
        Ok(())
    }
}

impl Meal {
    pub async fn delete_entry(&self, pool: &SqlitePool) -> Result<(), sqlx::Error> {

        sqlx::query!("DELETE FROM meals WHERE id = ?", self.id)
        .execute(pool)
        .await?; 
        println!("[INFO] Deleted entry with id = {}", self.id); 
        
        Ok(())
    }
}

impl DailyLog {
    pub async fn delete_entry(&self, pool: &SqlitePool) -> Result<(), sqlx::Error> {

        sqlx::query!("DELETE FROM daily_logs WHERE id = ?", self.id)
        .execute(pool)
        .await?; 

        println!("[INFO] Deleted entry with id = {}", self.id); 
        Ok(())
    }
}

impl Recipe {
    pub async fn delete_entry(&self, pool: &SqlitePool) -> Result<(), sqlx::Error> {

        sqlx::query!("DELETE FROM recipes WHERE id = ?", self.id)
        .execute(pool)
        .await?; 

        println!("[INFO] Deleted entry with id = {}", self.id); 
        Ok(())
    }
}

impl Ingredient {
    pub async fn delete_entry(&self, pool: &SqlitePool) -> Result<(), sqlx::Error> {

        sqlx::query!("DELETE FROM ingredients WHERE id = ?", self.id)
        .execute(pool)
        .await?; 

        println!("[INFO] Deleted entry with id = {}", self.id); 
        Ok(())
    }
}