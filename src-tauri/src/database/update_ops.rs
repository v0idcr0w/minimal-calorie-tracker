use sqlx::SqlitePool;

use super::super::models::{food::Food, meal::Meal, food_normalized::FoodNormalized, daily_log::DailyLog, ingredient::Ingredient, recipe::Recipe, macros_total::MacrosTotal}; 

impl Food {
    pub fn update(&mut self, new_amount: f64) {
        // updates the struct instance itself  
        // only allowed to provide the new amount. remaining parts are updated automatically. 
        // updating units requires updating FoodNormalized.  
        let old_amount = self.amount;  
        self.amount = new_amount; 
        self.protein *= new_amount/old_amount; 
        self.carbohydrate *= new_amount/old_amount; 
        self.fat *= new_amount/old_amount; 
        self.calories *= new_amount/old_amount; 
    }
    pub async fn update_entry(&self, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        // update database entry 
        sqlx::query!("UPDATE foods SET amount = ?, protein = ?, carbohydrate = ?, fat = ?, calories = ? WHERE id = ?", self.amount, self.protein, self.carbohydrate, self.fat, self.calories, self.id)
        .execute(pool)
        .await?; 
    
        println!("[INFO] Updated entry with id = {}", self.id); 
        Ok(())
    }
}

impl FoodNormalized {
    pub async fn update_name(&mut self, new_name: String, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        self.name = new_name.to_lowercase();
        sqlx::query!("UPDATE foods_normalized SET name = ? WHERE id = ?", self.name, self.id).execute(pool).await?;
        Ok(())
    }
    pub fn update(&mut self, new_food_normalized: FoodNormalized) {
        // new_food_normalized ceases to exist after this block ends (move) 
        self.unit = new_food_normalized.unit; 
        self.serving_size = new_food_normalized.serving_size; 
        self.normalized_protein = new_food_normalized.normalized_protein; 
        self.normalized_carbohydrate = new_food_normalized.normalized_carbohydrate; 
        self.normalized_fat = new_food_normalized.normalized_fat; 
        self.normalized_calories = new_food_normalized.normalized_calories; 
    }

    pub async fn update_entry(&self, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE foods_normalized SET name = ?, unit = ?, serving_size = ?, normalized_protein = ?, normalized_carbohydrate = ?, normalized_fat = ?, normalized_calories = ? WHERE id = ?", self.name, self.unit, self.serving_size, self.normalized_protein, self.normalized_carbohydrate, self.normalized_fat, self.normalized_calories, self.id)
        .execute(pool)
        .await?; 

        println!("[INFO] Updated entry with id = {}", self.id); 
        Ok(())
    }
}

impl Meal {
    pub fn update_name(&mut self, new_name: String) {
        self.name = new_name.to_lowercase(); 
    }
    pub async fn update_entry(&self, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        // updates only the meal type name. other updates are made through the food table. 
        sqlx::query!("UPDATE meals SET name = ? WHERE id = ?", self.name, self.id)
        .execute(pool)
        .await?;    

        println!("[INFO] Updated entry with id = {}", self.id); 
        Ok(())
    }
}

impl DailyLog {
    pub async fn update_entry(&self, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE daily_logs SET total_calories = ?, total_protein = ?, total_carbohydrate = ?, total_fat = ? WHERE id = ?", self.total_calories, self.total_protein, self.total_carbohydrate, self.total_fat, self.id)
        .execute(pool)
        .await?;        

        println!("[INFO] Updated entry with id = {}", self.id); 
        Ok(())
    }
    pub async fn update_weight(&self, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE daily_logs SET weight = ? WHERE id = ?", self.weight, self.id)
        .execute(pool)
        .await?;        

        println!("[INFO] Updated entry with id = {}", self.id); 
        Ok(())
    }
}

impl Recipe {
    pub async fn update_name(&mut self, new_name: String, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        // updates the recipe's name in the database 
        self.name = new_name.to_lowercase(); 
        sqlx::query!("UPDATE recipes SET name = ? WHERE id = ?", self.name, self.id).execute(pool).await?;
        Ok(()) 
    }
    pub fn update_serving_size(&mut self, new_serving_size: f64, new_unit: String) {
        // updates serving size and also the unit in the database 
        self.unit = new_unit; 
        let old_serving_size = self.serving_size; 
        self.serving_size = new_serving_size; 
        self.protein *= new_serving_size/old_serving_size; 
        self.carbohydrate *= new_serving_size/old_serving_size; 
        self.fat *= new_serving_size/old_serving_size; 
        self.calories *= new_serving_size/old_serving_size; 
    }
    pub fn update_macros(&mut self, new_macros: MacrosTotal) {
        // updates the macros and also the calories in the database 
        self.protein += new_macros.protein;
        self.carbohydrate += new_macros.carbohydrate; 
        self.fat += new_macros.fat;
        self.calories += new_macros.calories; 
    }
    pub async fn update_entry(&self, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        // updates macros/serving size/unit in the database 
        sqlx::query!("UPDATE recipes SET serving_size = ?, unit = ?, protein = ?, carbohydrate = ?, fat = ?, calories = ? WHERE id = ?", self.serving_size, self.unit, self.protein, self.carbohydrate, self.fat, self.calories, self.id)
        .execute(pool)
        .await?;        

        println!("[INFO] Updated entry with id = {}", self.id); 
        Ok(())
    }
}

impl Ingredient {
    pub fn update(&mut self, new_amount: f64) {
        let old_amount = self.amount; 
        self.amount = new_amount; 
        self.protein *= new_amount/old_amount; 
        self.carbohydrate *= new_amount/old_amount; 
        self.fat *= new_amount/old_amount; 
        self.calories *= new_amount/old_amount; 
    }
    pub async fn update_entry(&self, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE ingredients SET amount = ?, protein = ?, carbohydrate = ?, fat = ?, calories = ? WHERE id = ?", self.amount, self.protein, self.carbohydrate, self.fat, self.calories, self.id)
        .execute(pool)
        .await?;        

        println!("[INFO] Updated entry with id = {}", self.id); 
        Ok(())
    }
}