INSERT INTO foods_normalized (name, unit, serving_size, normalized_protein, normalized_carbohydrate, normalized_fat, normalized_calories) VALUES 
    ('oatmeal', 'g', 100, 3.2, 12.4, 1.3, 71.0),
    ('milk', 'ml', 100, 3.0, 4.5, 3.0, 60.0),
    ('banana', 'g', 100, 1.0, 27.0, 0.3, 105.0),
    ('rice', 'g', 100, 2.0, 28.0, 0.3, 130.0),
    ('chicken', 'g', 100, 31.0, 0.0, 3.6, 165.0),
    ('broccoli', 'g', 100, 2.8, 3.5, 0.4, 29.0),
    ('pasta', 'g', 100, 12.0, 75.0, 1.3, 370.0),
    ('tomato sauce', 'g', 100, 1.2, 6.0, 3.0, 50.0),
    ('parmesan', 'g', 100, 35.0, 3.0, 25.0, 400.0);


INSERT INTO daily_logs (weight, total_protein, total_carbohydrate, total_fat, total_calories) VALUES 
    (0, 0, 0, 0, 0);

INSERT INTO meals (name, log_id) VALUES 
    ('breakfast', 1),
    ('lunch', 1),
    ('dinner', 1); 

INSERT INTO foods (foods_normalized_id, meal_id, name, unit, amount, protein, carbohydrate, fat, calories) VALUES 
    (1, 1, 'oatmeal', 'g', 300, 9.6, 37.2, 3.9, 213.0),
    (2, 1, 'milk', 'ml', 200, 12.0, 18.0, 12.0, 240.0),
    (3, 1, 'banana', 'g', 150, 1.5, 40.5, 0.45, 157.5),
    (4, 2, 'rice', 'g', 133, 2.66, 37.24, 0.399, 172.9),
    (5, 2, 'chicken', 'g', 250, 77.5, 0.0, 9.0, 412.5),
    (6, 2, 'broccoli', 'g', 90, 2.52, 3.15, 0.36, 26.1),
    (7, 3, 'pasta', 'g', 250, 30.0, 187.5, 3.25, 925.0),
    (8, 3, 'tomato sauce', 'g', 200, 2.4, 12.0, 6.0, 100.0),
    (9, 3, 'parmesan', 'g', 10, 3.5, 0.3, 2.5, 40.0);
