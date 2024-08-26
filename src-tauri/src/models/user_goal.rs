use sqlx::{FromRow, SqlitePool}; 
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, FromRow)] 
pub struct UserGoal {
    pub id: i32, 
    pub weight: f64,
    pub protein: f64, 
    pub carbohydrate: f64, 
    pub fat: f64, 
    pub calories: f64
}

impl UserGoal {
    pub fn new(weight: f64, protein: f64, carbohydrate: f64, fat: f64, calories: f64) -> Self {
        UserGoal {
            id: 0, 
            weight, 
            protein, 
            carbohydrate, 
            fat, 
            calories
        }
    }
    pub async fn create(self, pool: &SqlitePool) -> Result<Self, sqlx::Error> {
        let result = sqlx::query_as("INSERT INTO user_goals (weight, protein, carbohydrate, fat, calories) VALUES (?, ?, ?, ?, ?) RETURNING *")
        .bind(self.weight)
        .bind(self.protein)
        .bind(self.carbohydrate)
        .bind(self.fat)
        .bind(self.calories)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }
    pub async fn update(self, db: &SqlitePool) -> Result<Self, sqlx::Error> {
        let result = sqlx::query_as("UPDATE user_goals SET weight = ?, protein = ?, carbohydrate = ?, fat = ?, calories = ? WHERE id = ? RETURNING *")
        .bind(self.weight)
        .bind(self.protein)
        .bind(self.carbohydrate)
        .bind(self.fat)
        .bind(self.calories)
        .bind(self.id)
        .fetch_one(db)
        .await?;

        Ok(result)
    }
    pub async fn update_numeric_field(self, field_name: &str, new_value: f64, pool: &SqlitePool) -> Result<Self, sqlx::Error> {
        let sql_statement = format!("UPDATE user_goals SET {} = ? WHERE id = ? RETURNING *", field_name);

        let result = sqlx::query_as(sql_statement.as_str())
        .bind(new_value)
        .bind(self.id)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }
    pub async fn get_by_id(pk: i32, pool: &SqlitePool) -> Result<Self, sqlx::Error> {
        let goal = sqlx::query_as::<_, Self>("SELECT * FROM user_goals WHERE id = ?")
        .bind(pk)
        .fetch_one(pool)
        .await?;
        Ok(goal)
    }
}