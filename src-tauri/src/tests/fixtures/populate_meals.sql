-- prepopulate the foods table 
INSERT INTO daily_logs (weight, total_protein, total_carbohydrate, total_fat, total_calories) VALUES 
    (0, 0, 0, 0, 0);

INSERT INTO meals (name, log_id) VALUES 
    ('breakfast', 1),
    ('lunch', 1),
    ('dinner', 1); 

