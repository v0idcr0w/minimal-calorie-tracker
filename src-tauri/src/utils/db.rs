use sqlx::{migrate::MigrateDatabase, sqlite::{SqliteConnectOptions, SqliteRow}, ConnectOptions, Row, Sqlite}; 

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
        Ok(true) => {
            println!("Database already exists"); 
            // check if column "is_constant" from "meals" exists 
            check_column().await?; 
        }, 
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}

async fn check_column() -> Result <(), sqlx::Error> {
    let mut conn = SqliteConnectOptions::new().filename("database.db").connect().await?; 
    let row: SqliteRow = sqlx::query("SELECT COUNT(*) FROM pragma_table_info('meals') WHERE name='is_constant'").fetch_one(&mut conn).await?;  
    let count: i32 = row.get(0); 
    if count == 0 {
        println!("Column 'is_constant' does not exist"); 
        // add column "is_constant" to "meals" 
        sqlx::query("ALTER TABLE meals ADD COLUMN is_constant BOOLEAN DEFAULT 0").execute(&mut conn).await?; 
        println!("Creation successful"); 
    } else {
        println!("Column 'is_constant' already exists, skipping creation"); 
    }
    Ok(())
}
