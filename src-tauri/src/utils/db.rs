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
            let query = "ALTER TABLE meals ADD COLUMN is_constant BOOLEAN DEFAULT 0";
            check_column("is_constant", query, "meals").await?; 
            // check if column "is_disabled" from "meals" exists 
            let query = "ALTER TABLE meals ADD COLUMN is_disabled BOOLEAN DEFAULT 0";
            check_column("is_disabled", query, "meals").await?; 
            // check if column "units" from "daily_logs" exists
            let query = "ALTER TABLE daily_logs ADD COLUMN units INTERGER DEFAULT 0";
            check_column("units", query, "daily_logs").await?; 
        }, 
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}

async fn check_column(column_name: &str, query: &str, table: &str) -> Result <(), sqlx::Error> {
    let mut conn = SqliteConnectOptions::new().filename("database.db").connect().await?; 
    let check = &format!("SELECT COUNT(*) FROM pragma_table_info('{table}') WHERE name='{column_name}'");
    let row: SqliteRow = sqlx::query(check).fetch_one(&mut conn).await?;  
    let count: i32 = row.get(0); 
    if count == 0 {
        println!("Column '{}' does not exist", column_name); 
        // add column "column_name" to "meals"
        sqlx::query(query).execute(&mut conn).await?; 
        println!("Creation successful"); 
    } else {
        println!("Column '{}' already exists, skipping creation", column_name); 
    }
    Ok(())
}
