use sqlx::{FromRow, SqlitePool}; 
use serde::{Serialize, Deserialize};
use super::macros_total::MacrosTotal;


#[derive(Debug, PartialEq, Serialize, Deserialize, FromRow)] 
pub struct Recipe {
    pub id: i32, 
    pub name: String, 
    pub serving_size: f64, 
    pub unit: String, 
    pub protein: f64, 
    pub carbohydrate: f64, 
    pub fat: f64, 
    pub calories: f64
}

impl Recipe {
    pub fn new(id: i32, name: String, serving_size: f64, unit: String, protein: f64, carbohydrate: f64, fat: f64, calories: f64) -> Self {
        Self { id, name, serving_size, unit, protein, carbohydrate, fat, calories }
    }
    pub async fn create(self, db: &SqlitePool) -> Result<Self, sqlx::Error> {
        let recipe = sqlx::query_as("INSERT INTO recipes (name, serving_size, unit, protein, carbohydrate, fat, calories) VALUES (?, ?, ?, ?, ?, ?, ?) RETURNING *")
        .bind(self.name.to_lowercase())
        .bind(self.serving_size)
        .bind(self.unit)
        .bind(self.protein)
        .bind(self.carbohydrate)
        .bind(self.fat)
        .bind(self.calories)
        .fetch_one(db)
        .await; 

        recipe
    }
    pub fn update_serving_size(mut self, new_serving_size: f64, new_unit: String) ->  Self {
        // updates serving size and also the unit in the database 
        self.unit = new_unit; 
        let old_serving_size = self.serving_size; 
        self.serving_size = new_serving_size; 
        self.protein *= new_serving_size/old_serving_size; 
        self.carbohydrate *= new_serving_size/old_serving_size; 
        self.fat *= new_serving_size/old_serving_size; 
        self.calories *= new_serving_size/old_serving_size; 
        
        self 
    }
    pub fn update_macros(mut self, new_macros: MacrosTotal) -> Self {
        // updates the macros and also the calories in the database 
        self.protein += new_macros.protein;
        self.carbohydrate += new_macros.carbohydrate; 
        self.fat += new_macros.fat;
        self.calories += new_macros.calories; 
        
        self 
    }
    pub async fn update(self, db: &SqlitePool) -> Result<Self, sqlx::Error> {
        let updated_recipe = sqlx::query_as("UPDATE recipes SET name = ?, serving_size = ?, unit = ?, protein = ?, carbohydrate = ?, fat = ?, calories = ? WHERE id = ? RETURNING *")
        .bind(self.name.to_lowercase())
        .bind(self.serving_size)
        .bind(self.unit)
        .bind(self.protein)
        .bind(self.carbohydrate)
        .bind(self.fat)
        .bind(self.calories)
        .bind(self.id)
        .fetch_one(db)
        .await;

        updated_recipe
    }
    pub async fn delete(self, db: &SqlitePool) -> Result<(), sqlx::Error> {
        let _ = sqlx::query("DELETE FROM recipes WHERE id = ?").bind(self.id).execute(db).await?;
        Ok(())
    }
    pub async fn get_by_id(id: i32, db: &SqlitePool) -> Result<Self, sqlx::Error> {
        let recipe = sqlx::query_as::<_, Self>("SELECT * FROM recipes WHERE id = ?")
        .bind(id)
        .fetch_one(db)
        .await?;
        Ok(recipe)
    }

    pub async fn get_all(db: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        let recipe_list = sqlx::query_as::<_, Self>("SELECT * FROM recipes")
        .fetch_all(db)
        .await?;
        Ok(recipe_list)
    }
}

