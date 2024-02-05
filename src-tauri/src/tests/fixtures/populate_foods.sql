-- prepopulate the foods table 
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
