-- Add migration script here
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


-- prepopulate the foods table 
INSERT INTO foods_normalized (name, unit, serving_size, normalized_protein, normalized_carbohydrate, normalized_fat, normalized_calories) VALUES 
    ('apple', 'g', 185, 0.5, 25, 0.3, 95),
    ('orange', 'g', 140, 1.3, 18, 0.2, 69),
    ('skim milk', 'mL', 237, 7.6, 13.2, 0, 83); 


-- prepopulate the foods table 
INSERT INTO foods (name, foods_normalized_id, unit, amount, protein, carbohydrate, fat, calories) VALUES 
    ('apple', 1, 'g', 370, 1.0, 50.0, 0.6, 190),
    ('orange', 2, 'g', 270, 2.6, 36, 0.4, 138),
    ('skim milk', 3, 'mL', 474, 15.2, 26.4, 0, 166); 

INSERT INTO meals (meal_type) VALUES ('breakfast'); 
INSERT INTO meals (meal_type) VALUES ('dinner'); 
INSERT INTO meals_foods (meal_id, food_id) VALUES (1, 1), (1, 2);
INSERT INTO meals_foods (meal_id, food_id) VALUES (2, 1), (2, 3); 

