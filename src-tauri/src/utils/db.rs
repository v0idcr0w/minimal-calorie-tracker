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
            check_column("is_constant").await?; 
            // check if column "is_disabled" from "meals" exists 
            check_column("is_disabled").await?; 
        }, 
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}

async fn check_column(column_name: &str) -> Result <(), sqlx::Error> {
    let mut conn = SqliteConnectOptions::new().filename("database.db").connect().await?; 
    let query = &format!("SELECT COUNT(*) FROM pragma_table_info('meals') WHERE name='{}'", column_name);
    let row: SqliteRow = sqlx::query(query).fetch_one(&mut conn).await?;  
    let count: i32 = row.get(0); 
    if count == 0 {
        println!("Column '{}' does not exist", column_name); 
        // add column "column_name" to "meals"
        let query = &format!("ALTER TABLE meals ADD COLUMN {} BOOLEAN DEFAULT 0", column_name); 
        sqlx::query(query).execute(&mut conn).await?; 
        println!("Creation successful"); 
    } else {
        println!("Column '{}' already exists, skipping creation", column_name); 
    }
    Ok(())
}
