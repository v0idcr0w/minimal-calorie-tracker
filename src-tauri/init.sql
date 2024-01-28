DROP TABLE IF EXISTS foods_normalized; 
DROP TABLE IF EXISTS foods;
DROP TABLE IF EXISTS meals;
DROP TABLE IF EXISTS daily_logs;
DROP TABLE IF EXISTS meals_foods; 

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

CREATE TABLE foods (
    id INTEGER PRIMARY KEY, 
    foods_normalized_id INTEGER NOT NULL, 
    name TEXT NOT NULL, 
    unit TEXT NOT NULL, 
    amount REAL NOT NULL, 
    protein REAL NOT NULL,
    carbohydrate REAL NOT NULL,
    fat REAL NOT NULL, 
    calories REAL NOT NULL,
    FOREIGN KEY (foods_normalized_id) REFERENCES foods_normalized(id) ON DELETE CASCADE
);

CREATE TABLE meals (
    id INTEGER PRIMARY KEY, 
    meal_type TEXT NOT NULL
); 

CREATE TABLE daily_logs (
    id INTEGER PRIMARY KEY, 
    weight REAL NOT NULL, -- the user's current weight 
    total_protein REAL NOT NULL,
    total_carbohydrate REAL NOT NULL,
    total_fat REAL NOT NULL, 
    total_calories REAL NOT NULL,  
    current_date DATE NOT NULL
);


-- junction table for many-to-many relationship between foods and meals 
CREATE TABLE meals_foods (
    meal_id INTEGER NOT NULL REFERENCES meals(id) ON DELETE CASCADE, 
    food_id INTEGER NOT NULL REFERENCES foods(id) ON DELETE CASCADE, 
    PRIMARY KEY (meal_id, food_id)
);
