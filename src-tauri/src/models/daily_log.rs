use sqlx::{FromRow, SqlitePool, types::chrono::NaiveDate, Type}; 
use serde::{Serialize, Deserialize};
use super::macros_total::MacrosTotal;

#[derive(Debug, PartialEq, Serialize, Deserialize, FromRow)] 
pub struct DailyLog {
    pub id: i32, 
    pub weight: f64,
    pub units: WeightUnits,
    pub total_protein: f64, 
    pub total_carbohydrate: f64, 
    pub total_fat: f64, 
    pub total_calories: f64,
    pub entry_date: NaiveDate, 
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Type)]
#[repr(i32)]
pub enum WeightUnits {
    Kg = 0,
    Lbs = 1,
}

impl DailyLog {
    pub fn new(entry_date: NaiveDate, units: WeightUnits) -> Self {
        Self { id: 0, units, weight: 0.0, total_protein: 0.0, total_carbohydrate: 0.0, total_fat: 0.0, total_calories: 0.0, entry_date }
    }
    pub fn update_macros(mut self, macros: MacrosTotal) -> Self {
        self.total_protein = macros.protein; 
        self.total_carbohydrate = macros.carbohydrate; 
        self.total_fat = macros.fat; 
        self.total_calories = macros.calories; 
        self 
    }
    pub async fn create(self, db: &SqlitePool) -> Result<Self, sqlx::Error> {
        let log = sqlx::query_as("INSERT INTO daily_logs (weight, total_protein, total_carbohydrate, total_fat, total_calories, entry_date) VALUES (?, ?, ?, ?, ?, ?) RETURNING *")
        .bind(self.weight)
        .bind(self.total_protein)
        .bind(self.total_carbohydrate)
        .bind(self.total_fat)
        .bind(self.total_calories)
        .bind(self.entry_date)
        .fetch_one(db)
        .await; 

        log
    }
    pub async fn update_weight(self, new_weight: f64, new_units: WeightUnits, db: &SqlitePool) -> Result<Self, sqlx::Error> {
        let updated_log = sqlx::query_as("UPDATE daily_logs SET weight = ?, units = ? WHERE id = ? RETURNING *")
        .bind(new_weight)
        .bind(new_units)
        .bind(self.id)
        .fetch_one(db)
        .await;

        updated_log
    }
    pub async fn update(self, db: &SqlitePool) -> Result<Self, sqlx::Error> {
        let updated_log = sqlx::query_as("UPDATE daily_logs SET weight = ?, total_protein = ?, total_carbohydrate = ?, total_fat = ?, total_calories = ? WHERE id = ? RETURNING *")
        .bind(self.weight)
        .bind(self.total_protein)
        .bind(self.total_carbohydrate)
        .bind(self.total_fat)
        .bind(self.total_calories)
        .bind(self.id)
        .fetch_one(db)
        .await;

        updated_log
    }
    pub async fn delete(self, db: &SqlitePool) -> Result<(), sqlx::Error> {
        let _ = sqlx::query("DELETE FROM daily_logs WHERE id = ?").bind(self.id).execute(db).await?;
        Ok(())
    }
    pub async fn get_by_id(id: i32, db: &SqlitePool) -> Result<Self, sqlx::Error> {
        let logs = sqlx::query_as::<_, Self>("SELECT * FROM daily_logs WHERE id = ?")
        .bind(id)
        .fetch_one(db)
        .await?;
    Ok(logs)
    }
    
    pub async fn get_all(db: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        let logs_list = sqlx::query_as::<_, Self>("SELECT * FROM daily_logs")
        .fetch_all(db)
        .await?;
        Ok(logs_list)
    }

    pub async fn get_by_date(date: NaiveDate, db: &SqlitePool) -> Result<Self, sqlx::Error> {
        let log = sqlx::query_as::<_, Self>("SELECT * FROM daily_logs WHERE entry_date = ?")
        .bind(date)
        .fetch_one(db)
        .await?;
        Ok(log) 
    }
}