-- Add migration script here
DROP TABLE IF EXISTS user_goal; 

CREATE TABLE user_goals (
    id INTEGER PRIMARY KEY, 
    weight TEXT, 
    weight_rate REAL, 
    protein REAL, 
    carbohydrate REAL, 
    fat REAL, 
    calories REAL 
); 