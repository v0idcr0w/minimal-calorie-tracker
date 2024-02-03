-- prepopulate the foods table 
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