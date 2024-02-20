use sqlx::{migrate::MigrateDatabase, sqlite::SqliteConnectOptions, Sqlite, ConnectOptions}; 

pub const DB_URL: &str = "sqlite://database.db"; 

pub async fn create(content: &str) -> Result <(), sqlx::Error> {
    match Sqlite::database_exists(DB_URL).await {
        Ok(false) => {
            // create database 
            let mut conn = SqliteConnectOptions::new()
            .filename("database.db")
            .create_if_missing(true)
            .connect().await?; 
            // create tables
            sqlx::query(content).execute(&mut conn).await?;
        }
        Ok(true) => println!("Database already exists"), 
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}