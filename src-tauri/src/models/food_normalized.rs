use sqlx::{FromRow, SqlitePool}; 
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, FromRow)] 
pub struct FoodNormalized {
    pub id: i32, 
    pub name: String, 
    pub serving_size: f64, 
    pub unit: String, 
    pub normalized_protein: f64, 
    pub normalized_carbohydrate: f64, 
    pub normalized_fat: f64, 
    pub normalized_calories: f64
}

#[derive(Debug, Deserialize, Serialize)] 
pub struct FoodNormalizedCsv {
    // this struct is the same as the previous struct, but it's used to serialize/deserialize the CSV file, and doesn't have an id. 
    pub name: String, 
    pub serving_size: f64, 
    pub unit: String, 
    pub normalized_protein: f64, 
    pub normalized_carbohydrate: f64, 
    pub normalized_fat: f64, 
    pub normalized_calories: f64    
}

impl From<FoodNormalized> for FoodNormalizedCsv {
    fn from(food: FoodNormalized) -> Self {
        Self { name: food.name, serving_size: food.serving_size, unit: food.unit, normalized_protein: food.normalized_protein, normalized_carbohydrate: food.normalized_carbohydrate, normalized_fat: food.normalized_fat, normalized_calories: food.normalized_calories }
    }
}

impl From<FoodNormalizedCsv> for FoodNormalized {
    fn from(food: FoodNormalizedCsv) -> Self {
        Self { id: 0, name: food.name, serving_size: food.serving_size, unit: food.unit, normalized_protein: food.normalized_protein, normalized_carbohydrate: food.normalized_carbohydrate, normalized_fat: food.normalized_fat, normalized_calories: food.normalized_calories }
    }
}


impl FoodNormalized {
    pub async fn create(self, db: &SqlitePool) -> Result<Self, sqlx::Error> {
        let food: Result<Self, _> = sqlx::query_as("INSERT INTO foods_normalized (name, serving_size, unit, normalized_protein, normalized_carbohydrate, normalized_fat, normalized_calories) VALUES (?, ?, ?, ?, ?, ?, ?) RETURNING *")
        .bind(self.name.to_lowercase())
        .bind(self.serving_size)
        .bind(self.unit)
        .bind(self.normalized_protein)
        .bind(self.normalized_carbohydrate)
        .bind(self.normalized_fat)
        .bind(self.normalized_calories)
        .fetch_one(db)
        .await;
        food
    }
    pub async fn update_name(self, new_name: &str, db: &SqlitePool) -> Result<Self, sqlx::Error> {
        let updated_food: Result<Self, _> = sqlx::query_as("UPDATE foods_normalized SET name = ? WHERE id = ? RETURNING *")
        .bind(new_name.to_lowercase())
        .bind(self.id)
        .fetch_one(db)
        .await;
        updated_food
    }
    pub async fn update(self, db: &SqlitePool) -> Result<Self, sqlx::Error> {
        let updated_food: Result<Self, _> = sqlx::query_as("UPDATE foods_normalized SET serving_size = ?, unit = ?, normalized_protein = ?, normalized_carbohydrate = ?, normalized_fat = ?, normalized_calories = ? WHERE id = ? RETURNING *")
        .bind(self.serving_size)
        .bind(self.unit)
        .bind(self.normalized_protein)
        .bind(self.normalized_carbohydrate)
        .bind(self.normalized_fat)
        .bind(self.normalized_calories)
        .bind(self.id)
        .fetch_one(db)
        .await;
        updated_food 
    }
    pub async fn delete(self, db: &SqlitePool) -> Result<(), sqlx::Error> {
        let _ = sqlx::query("DELETE FROM foods_normalized WHERE id = ?")
        .bind(self.id)
        .execute(db)
        .await?;

        Ok(())
    }
    pub async fn get_by_id(id: i32, db: &SqlitePool) -> Result<Self, sqlx::Error> {
        let food: Result<Self, _> = sqlx::query_as("SELECT * FROM foods_normalized WHERE id = ?")
        .bind(id)
        .fetch_one(db)
        .await;
        food
    }
    pub async fn get_all(db: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        let foods = sqlx::query_as("SELECT * FROM foods_normalized")
        .fetch_all(db)
        .await; 
        foods 
    }
}