use sqlx::{SqlitePool, Sqlite, migrate::MigrateDatabase}; 
// database initialization 
pub const DB_URL: &str = "sqlite://database.db";

#[allow(dead_code)]
pub async fn create_db() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("[INFO] Create db success"),
            Err(error) => panic!("Error: {}", error),
        }
    } else {
        println!("[INFO] Database already exists");
    }
}

// pub async fn prepopulate_db(pool: &SqlitePool) {
//     let result  = sqlx::query("INSERT INTO foods_normalized (name, unit, serving_size, normalized_protein, normalized_carbohydrate, normalized_fat, normalized_calories) VALUES 
//     ('apple', 'g', 185, 0.5, 25, 0.3, 95),
//     ('orange', 'g', 140, 1.3, 18, 0.2, 69),
//     ('skim milk', 'mL', 237, 7.6, 13.2, 0, 83);")
//     .execute(pool)
//     .await; 
//     match result {
//         Ok(_) => println!("[INFO] Successfully prepopulated the database"),
//         Err(e) => panic!("Error: {}", e) 
//     }
// }

// pub async fn make_migrations(pool: &SqlitePool) {
//     let result = sqlx::migrate!("./migrations")
//     .run(pool)
//     .await;

//     match result {
//         Ok(_) => println!("[INFO] Migration successful"),
//         Err(e) => panic!("Error: {}", e) 
//     }

// }