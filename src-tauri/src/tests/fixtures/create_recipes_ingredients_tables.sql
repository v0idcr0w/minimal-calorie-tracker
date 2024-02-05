DROP TABLE IF EXISTS ingredients; 
DROP TABLE IF EXISTS recipes; 
DROP TABLE IF EXISTS meals_recipes; 

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

CREATE TABLE meals_recipes (
    meal_id INTEGER, 
    recipe_id INTEGER, 
    FOREIGN KEY (meal_id) REFERENCES meals(id) ON DELETE CASCADE,
    FOREIGN KEY (recipe_id) REFERENCES recipes(id) ON DELETE CASCADE,
    PRIMARY KEY (meal_id, recipe_id)
);