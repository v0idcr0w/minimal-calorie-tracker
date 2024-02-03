use sqlx::SqlitePool;
use super::super::models::{food::Food, meal::Meal, food_normalized::FoodNormalized, daily_log::DailyLog}; 

impl FoodNormalized {
    pub async fn create_entry(&mut self, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        // example usage: create Food::new() with dummy ids, turn into FoodNormalize via the from method, then use create entry method to add to the database. 
        self.name = self.name.to_lowercase(); // ensure this is lowercase 
        let result = sqlx::query!("INSERT INTO foods_normalized (name, unit, serving_size, normalized_protein, normalized_carbohydrate, normalized_fat, normalized_calories) VALUES (?, ?, ?, ?, ?, ?, ?)",
        self.name, self.unit, self.serving_size, self.normalized_protein, self.normalized_carbohydrate, self.normalized_fat, self.normalized_calories)
        .execute(pool)
        .await?;

        self.id = result.last_insert_rowid() as i32; 

        println!("[INFO] Inserted new entry"); 
        Ok(())
    }
}

impl Food {
    pub async fn create_entry(&mut self, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        self.name = self.name.to_lowercase(); 
        let result = sqlx::query!("INSERT INTO foods (foods_normalized_id, meal_id, name, unit, amount, protein, carbohydrate, fat, calories) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        self.foods_normalized_id, self.meal_id, self.name, self.unit, self.amount, self.protein, self.carbohydrate, self.fat, self.calories)
        .execute(pool)
        .await?; 

        // Get the last inserted row's ID using LAST_INSERT_ROWID()

        self.id = result.last_insert_rowid() as i32; 

        // print msg 
        println!("[INFO] Inserted new entry"); 

        Ok(())
    }
}

impl Meal {
    pub async fn create_entry(&mut self, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        self.name = self.name.to_lowercase(); 
        // creates a meal entry without any foods associated
        let result = sqlx::query!("INSERT INTO meals (name, entry_timestamp, log_id) VALUES (?, ?, ?)",
        self.name, self.entry_timestamp, self.log_id)
        .execute(pool)
        .await?;

        self.id = result.last_insert_rowid() as i32;

        Ok(())
    }

}

impl DailyLog {
    pub async fn create_entry(&mut self, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        let result = sqlx::query!("INSERT INTO daily_logs (weight, total_protein, total_carbohydrate, total_fat, total_calories, entry_date) VALUES (?, ?, ?, ?, ?, ?)",
        self.weight, self.total_protein, self.total_carbohydrate, self.total_fat, self.total_calories, self.entry_date)
        .execute(pool)
        .await?;        

        self.id = result.last_insert_rowid() as i32; 

        println!("[INFO] Inserted new entry"); 
        Ok(())
    }
}
