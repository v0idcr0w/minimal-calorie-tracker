use sqlx::SqlitePool;
use super::super::models::{food::Food, meal::Meal, food_normalized::FoodNormalized, food_normalized::FoodNormalizedCsv, daily_log::DailyLog, ingredient::Ingredient, recipe::Recipe, user_goal::UserGoal}; 

impl FoodNormalized {
    pub async fn create_entry(mut self, pool: &SqlitePool) -> Result<Self, sqlx::Error> {
        self.name = self.name.to_lowercase(); // ensure this is lowercase 
        let result = sqlx::query!("INSERT INTO foods_normalized (name, unit, serving_size, normalized_protein, normalized_carbohydrate, normalized_fat, normalized_calories) VALUES (?, ?, ?, ?, ?, ?, ?)",
        self.name, self.unit, self.serving_size, self.normalized_protein, self.normalized_carbohydrate, self.normalized_fat, self.normalized_calories)
        .execute(pool)
        .await?;

        self.id = result.last_insert_rowid() as i32; 

        println!("[INFO] Inserted new entry"); 
        Ok(self)
    }
}

impl FoodNormalizedCsv {
    pub async fn create_entry(self, pool: &SqlitePool) -> Result<Self, sqlx::Error> {
        let _result = sqlx::query!("INSERT INTO foods_normalized (name, unit, serving_size, normalized_protein, normalized_carbohydrate, normalized_fat, normalized_calories) VALUES (?, ?, ?, ?, ?, ?, ?)",
        self.name, self.unit, self.serving_size, self.normalized_protein, self.normalized_carbohydrate, self.normalized_fat, self.normalized_calories)
        .execute(pool)
        .await?;

        println!("[INFO] Inserted new entry"); 
        Ok(self)
    }
}

impl Food {
    pub async fn create_entry(mut self, pool: &SqlitePool) -> Result<Self, sqlx::Error> {
        self.name = self.name.to_lowercase(); 
        let result = sqlx::query!("INSERT INTO foods (food_normalized_id, recipe_id, meal_id, name, unit, amount, protein, carbohydrate, fat, calories) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        self.food_normalized_id, self.recipe_id, self.meal_id, self.name, self.unit, self.amount, self.protein, self.carbohydrate, self.fat, self.calories)
        .execute(pool)
        .await?; 

        // Get the last inserted row's ID using LAST_INSERT_ROWID()

        self.id = result.last_insert_rowid() as i32; 

        // print msg 
        println!("[INFO] Inserted new entry"); 

        Ok(self)
    }
}

impl Meal {
    pub async fn create_entry(mut self, pool: &SqlitePool) -> Result<Self, sqlx::Error> {
        self.name = self.name.to_lowercase(); 
        // creates a meal entry without any foods associated
        let result = sqlx::query!("INSERT INTO meals (name, entry_timestamp, log_id, is_constant) VALUES (?, ?, ?, ?)",
        self.name, self.entry_timestamp, self.log_id, self.is_constant)
        .execute(pool)
        .await?;

        self.id = result.last_insert_rowid() as i32;

        Ok(self)
    }
}

impl DailyLog {
    pub async fn create_entry(mut self, pool: &SqlitePool) -> Result<Self, sqlx::Error> {
        let result = sqlx::query!("INSERT INTO daily_logs (weight, total_protein, total_carbohydrate, total_fat, total_calories, entry_date) VALUES (?, ?, ?, ?, ?, ?)",
        self.weight, self.total_protein, self.total_carbohydrate, self.total_fat, self.total_calories, self.entry_date)
        .execute(pool)
        .await?;        

        self.id = result.last_insert_rowid() as i32; 

        println!("[INFO] Inserted new entry"); 
        Ok(self)
    }
}

impl Recipe {
    pub async fn create_entry(mut self, pool: &SqlitePool) -> Result<Self, sqlx::Error> {
        self.name = self.name.to_lowercase(); 
        let result = sqlx::query!("INSERT INTO recipes (name, serving_size, unit, protein, carbohydrate, fat, calories) VALUES (?, ?, ?, ?, ?, ?, ?)",
        self.name, self.serving_size, self.unit, self.protein, self.carbohydrate, self.fat, self.calories)
        .execute(pool)
        .await?;

        self.id = result.last_insert_rowid() as i32; 

        println!("[INFO] Inserted new entry"); 
        Ok(self)
    }
}

impl Ingredient {
    pub async fn create_entry(mut self, pool: &SqlitePool) -> Result<Self, sqlx::Error> {
        self.name = self.name.to_lowercase(); 
        let result = sqlx::query!("INSERT INTO ingredients (recipe_id, food_normalized_id, name, amount, unit, protein, carbohydrate, fat, calories) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        self.recipe_id, self.food_normalized_id, self.name, self.amount, self.unit, self.protein, self.carbohydrate, self.fat, self.calories)
        .execute(pool)
        .await?;

        self.id = result.last_insert_rowid() as i32; 

        println!("[INFO] Inserted new entry"); 
        Ok(self)
    }
}

impl UserGoal {
    pub async fn create_entry(mut self, pool: &SqlitePool) -> Result<Self, sqlx::Error> {
        let result = sqlx::query!("INSERT INTO user_goals (weight, protein, carbohydrate, fat, calories) VALUES (?, ?, ?, ?, ?)",
        self.weight, self.protein, self.carbohydrate, self.fat, self.calories)
        .execute(pool)
        .await?;

        self.id = result.last_insert_rowid() as i32; 

        println!("[INFO] Inserted new entry"); 
        Ok(self)
    }
}