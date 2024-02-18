-- Add migration script here
CREATE TABLE user_goals_new (
    id INTEGER PRIMARY KEY, 
    protein REAL, 
    carbohydrate REAL, 
    fat REAL, 
    calories REAL 
); 

INSERT INTO user_goals_new SELECT id, protein, carbohydrate, fat, calories FROM user_goals; 

DROP TABLE user_goals; 

ALTER TABLE user_goals_new RENAME TO user_goals; 

ALTER TABLE user_goals ADD COLUMN weight REAL; 
