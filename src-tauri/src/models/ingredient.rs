use sqlx::{FromRow, Sqlite, SqlitePool}; 
use serde::{Serialize, Deserialize};
use super::{food_normalized::FoodNormalized, macros_total::MacrosTotal};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)] 
pub struct Ingredient {
    pub id: i32, 
    pub recipe_id: i32, 
    pub food_normalized_id: i32, 
    pub name: String,
    pub amount: f64, 
    pub unit: String, 
    pub protein: f64,
    pub carbohydrate: f64,
    pub fat: f64,
    pub calories: f64
}

impl Ingredient {

    pub fn from(food_normalized: FoodNormalized, recipe_id: i32, amount: f64) -> Self {
        // initializes a new instance of Ingredient by taking some amount and multiplying macros by the correct value 
        let multiplier = amount / food_normalized.serving_size; 
        Self { 
            id: 0, // set id to zero 
            recipe_id,
            food_normalized_id: food_normalized.id, 
            name: food_normalized.name, 
            amount, 
            unit: food_normalized.unit, 
            protein: food_normalized.normalized_protein * multiplier, 
            carbohydrate: food_normalized.normalized_carbohydrate * multiplier, 
            fat: food_normalized.normalized_fat * multiplier, 
            calories: food_normalized.normalized_calories * multiplier 
        } 
    }

    pub fn into_macros_total(&self) -> MacrosTotal {
        // converts the ingredient into a MacrosTotal instance 
        MacrosTotal::new(self.protein, self.carbohydrate, self.fat, self.calories)
    }
    pub async fn create(self, db: &SqlitePool) -> Result<Self, sqlx::Error> {
        let ingredient = sqlx::query_as("INSERT INTO ingredients (recipe_id, food_normalized_id, name, amount, unit, protein, carbohydrate, fat, calories) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING *")
        .bind(self.recipe_id)
        .bind(self.food_normalized_id)
        .bind(self.name)
        .bind(self.amount)
        .bind(self.unit)
        .bind(self.protein)
        .bind(self.carbohydrate)
        .bind(self.fat)
        .bind(self.calories)
        .fetch_one(db)
        .await; 

        ingredient
    }
    pub async fn update(self, new_amount: f64, as_food_normalized: FoodNormalized, db: &SqlitePool) -> Result<Self, sqlx::Error> {
        let baseline_amount = as_food_normalized.serving_size;
         
        let new_ingredient = sqlx::query_as("UPDATE ingredients SET amount = ?, protein = ?, carbohydrate = ?, fat = ?, calories = ? WHERE id = ? RETURNING *")
        .bind(new_amount)
        .bind(as_food_normalized.normalized_protein * new_amount / baseline_amount)
        .bind(as_food_normalized.normalized_carbohydrate * new_amount / baseline_amount)
        .bind(as_food_normalized.normalized_fat * new_amount / baseline_amount)
        .bind(as_food_normalized.normalized_calories * new_amount / baseline_amount)
        .bind(self.id)
        .fetch_one(db)
        .await;
        new_ingredient
    }
    pub async fn delete(self, db: &SqlitePool) -> Result<(), sqlx::Error> {
        let _ = sqlx::query("DELETE FROM ingredients WHERE id = ?").bind(self.id).execute(db).await?;
        Ok(())
    }
    pub async fn get_by_id(id: i32, db: &SqlitePool) -> Result<Self, sqlx::Error> {
        let ingredient = sqlx::query_as::<_, Self>("SELECT * FROM ingredients WHERE id = ?")
        .bind(id)
        .fetch_one(db)
        .await?;
        Ok(ingredient)
    }

    pub async fn get_by_recipe_id(recipe_id: i32, db: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        let ingredients = sqlx::query_as::<_, Self>("SELECT * FROM ingredients WHERE recipe_id = ?")
        .bind(recipe_id)
        .fetch_all(db)
        .await?;
        Ok(ingredients)
    }

}