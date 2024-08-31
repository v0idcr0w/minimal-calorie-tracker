use sqlx::{FromRow, SqlitePool, Sqlite, Transaction}; 
use serde::{Serialize, Deserialize}; 
use super::{food_normalized::FoodNormalized, recipe::Recipe}; 
#[derive(Debug, Serialize, Deserialize, PartialEq, FromRow)] 
pub struct Food {
   pub id: i32, 
   pub food_normalized_id: Option<i32>, 
   pub recipe_id: Option<i32>, 
   pub meal_id: i32, 
   pub name: String, 
   pub unit: String,
   pub amount: f64, 
   pub protein: f64, 
   pub carbohydrate: f64,
   pub fat: f64, 
   pub calories: f64 
}

impl Food {
    pub fn from_food_normalized(food_normalized: FoodNormalized, meal_id: i32, amount: f64, id: i32) -> Self {
        // initializes a new instance of Food by taking some amount and multiplying macros by the correct value 
        let multiplier = amount / food_normalized.serving_size; 
        Self { 
            id, // set id to zero 
            food_normalized_id: Some(food_normalized.id), 
            recipe_id: None,  // null recipe id 
            meal_id,
            name: food_normalized.name, 
            unit: food_normalized.unit, 
            amount, 
            protein: food_normalized.normalized_protein * multiplier, 
            carbohydrate: food_normalized.normalized_carbohydrate * multiplier, fat: food_normalized.normalized_fat * multiplier, 
            calories: food_normalized.normalized_calories * multiplier } 
    }

    pub fn from_recipe(recipe: Recipe, meal_id: i32, amount: f64, id: i32) -> Self {
        // initializes a new instance of Food by taking some amount and multiplying macros by the correct value 
        let multiplier = amount / recipe.serving_size; 
        Self { 
            id, // set id to zero 
            food_normalized_id: None, 
            recipe_id: Some(recipe.id), 
            meal_id,
            name: recipe.name, 
            unit: recipe.unit, 
            amount, 
            protein: recipe.protein * multiplier, 
            carbohydrate: recipe.carbohydrate * multiplier, 
            fat: recipe.fat * multiplier, 
            calories: recipe.calories * multiplier 
        } 
    }
    pub async fn create<'a>(self, tx: &mut Transaction<'a, Sqlite>) -> Result<Self, sqlx::Error> {
        let food: Result<Self, _> = sqlx::query_as("INSERT INTO foods (food_normalized_id, recipe_id, meal_id, name, unit, amount, protein, carbohydrate, fat, calories) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING *")
        .bind(self.food_normalized_id)
        .bind(self.recipe_id)
        .bind(self.meal_id)
        .bind(self.name)
        .bind(self.unit)
        .bind(self.amount)
        .bind(self.protein)
        .bind(self.carbohydrate)
        .bind(self.fat)
        .bind(self.calories)
        .fetch_one(&mut **tx)
        .await;
        food
    }
    pub async fn update(self, new_amount: f64, db: &SqlitePool) -> Result<Self, sqlx::Error> {
        let updated_food = sqlx::query_as("UPDATE foods SET amount = ?, protein = ?, carbohydrate = ?, fat = ?, calories = ? WHERE id = ? RETURNING *")
        .bind(new_amount)
        .bind(self.protein * new_amount/self.amount)
        .bind(self.carbohydrate * new_amount/self.amount)
        .bind(self.fat * new_amount/self.amount)
        .bind(self.calories * new_amount/self.amount)
        .bind(self.id)
        .fetch_one(db)
        .await;
        updated_food 
    }
    pub async fn delete(self, db: &SqlitePool) -> Result<(), sqlx::Error> {
        let _ = sqlx::query("DELETE FROM foods WHERE id = ?").bind(self.id).execute(db).await?;
        Ok(())
    }
    pub async fn get_by_id(pk: i32, pool: &SqlitePool) -> Result<Self, sqlx::Error> {
        let food = sqlx::query_as::<_, Self>("SELECT * FROM foods WHERE id = ?")
        .bind(pk)
        .fetch_one(pool)
        .await?;
        Ok(food)
    }

    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        let food_list = sqlx::query_as::<_, Self>("SELECT * FROM foods")
        .fetch_all(pool)
        .await?;
        Ok(food_list)
    }

    pub async fn get_by_meal_id(meal_id: i32, pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        let foods = sqlx::query_as::<_, Self>("SELECT * FROM foods WHERE meal_id = ?")
        .bind(meal_id)
        .fetch_all(pool)
        .await?;
        Ok(foods)
    }
}