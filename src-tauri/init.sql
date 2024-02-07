DROP TABLE IF EXISTS foods_normalized; 
DROP TABLE IF EXISTS foods;
DROP TABLE IF EXISTS meals;
DROP TABLE IF EXISTS daily_logs;
DROP TABLE IF EXISTS user_goal; 
DROP TABLE IF EXISTS recipes; 
DROP TABLE IF EXISTS ingredients; 


CREATE TABLE user_goal (
    id INTEGER PRIMARY KEY, 
    weight REAL, 
    weight_rate REAL, 
    protein REAL, 
    carbohydrate REAL, 
    fat REAL, 
    calories REAL 
); 

CREATE TABLE foods_normalized (
    id INTEGER PRIMARY KEY, 
    name TEXT NOT NULL, 
    serving_size REAL NOT NULL, 
    unit TEXT NOT NULL, 
    normalized_protein REAL NOT NULL,
    normalized_carbohydrate REAL NOT NULL,
    normalized_fat REAL NOT NULL, 
    normalized_calories REAL NOT NULL
);

CREATE TABLE daily_logs (
    id INTEGER PRIMARY KEY, 
    weight REAL, -- the user's current weight 
    total_protein REAL, 
    total_carbohydrate REAL, 
    total_fat REAL,
    total_calories REAL, 
    entry_date TEXT DEFAULT CURRENT_DATE -- date in the ISO 8601 format "YYYY-MM-DD"
);

CREATE TABLE meals (
    id INTEGER PRIMARY KEY, 
    name TEXT NOT NULL, -- the meal name 
    log_id INTEGER NOT NULL, 
    entry_timestamp TEXT DEFAULT CURRENT_TIMESTAMP, -- date in the ISO 8601 format "YYYY-MM-DD HH:MM:SS.SSS"
    FOREIGN KEY (log_id) REFERENCES daily_logs(id) ON DELETE CASCADE
); 

CREATE TABLE recipes (
    id INTEGER PRIMARY KEY, 
    name TEXT NOT NULL,
    serving_size REAL,
    unit TEXT,  
    protein REAL,
    carbohydrate REAL,
    fat REAL, 
    calories REAL
);

CREATE TABLE foods (
    id INTEGER PRIMARY KEY, 
    food_normalized_id INTEGER, 
    recipe_id INTEGER, 
    meal_id INTEGER NOT NULL, 
    name TEXT NOT NULL, 
    unit TEXT NOT NULL, 
    amount REAL NOT NULL, 
    protein REAL NOT NULL,
    carbohydrate REAL NOT NULL,
    fat REAL NOT NULL, 
    calories REAL NOT NULL,
    FOREIGN KEY (food_normalized_id) REFERENCES foods_normalized(id) ON DELETE CASCADE, 
    FOREIGN KEY (recipe_id) REFERENCES recipes(id) ON DELETE CASCADE,
    FOREIGN KEY (meal_id) REFERENCES meals(id) ON DELETE CASCADE
);

CREATE TABLE ingredients (
    id INTEGER PRIMARY KEY, 
    recipe_id INTEGER NOT NULL,
    food_normalized_id INTEGER NOT NULL,  
    name TEXT NOT NULL, 
    amount REAL NOT NULL, 
    unit TEXT NOT NULL, 
    protein REAL NOT NULL,
    carbohydrate REAL NOT NULL,
    fat REAL NOT NULL, 
    calories REAL NOT NULL,
    FOREIGN KEY (recipe_id) REFERENCES recipes(id) ON DELETE CASCADE
    FOREIGN KEY (food_normalized_id) REFERENCES foods_normalized(id) ON DELETE CASCADE
);
