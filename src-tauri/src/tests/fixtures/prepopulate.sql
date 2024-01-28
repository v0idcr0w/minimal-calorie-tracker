-- prepopulate the foods table 
INSERT INTO foods_normalized (name, unit, serving_size, normalized_protein, normalized_carbohydrate, normalized_fat, normalized_calories) VALUES 
    ('apple', 'g', 185, 0.5, 25, 0.3, 95),
    ('orange', 'g', 140, 1.3, 18, 0.2, 69),
    ('skim milk', 'mL', 237, 7.6, 13.2, 0, 83); 
