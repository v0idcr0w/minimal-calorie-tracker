use sqlx::SqlitePool;
use super::super::models::{food::Food, meal::Meal, food_normalized::FoodNormalized, daily_log::DailyLog}; 

impl Food {
    pub async fn update(&mut self, new_amount: f64) {
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
    pub async fn update(&mut self, new_food_normalized: FoodNormalized) {
        // new_food_normalized ceases to exist after this block ends (move) 
        self.name = new_food_normalized.name; 
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
    pub async fn update(&mut self, new_type: String) {
        self.meal_type = new_type; 
    }
    pub async fn update_entry(&self, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        // updates only the meal type name. other updates are made through the food table. 
        sqlx::query!("UPDATE meals SET meal_type = ? WHERE id = ?", self.meal_type, self.id)
        .execute(pool)
        .await?;    

        println!("[INFO] Updated entry with id = {}", self.id); 
        Ok(())
    }
}

impl DailyLog {
    pub async fn update(&mut self, new_weight: f64) {
        // only weight can suffer changes. 
        self.weight = new_weight; 
    }
    pub async fn update_entry(&self, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE daily_logs SET weight = ? WHERE id = ?", self.weight, self.id)
        .execute(pool)
        .await?;        

        println!("[INFO] Updated entry with id = {}", self.id); 
        Ok(())
    }
}