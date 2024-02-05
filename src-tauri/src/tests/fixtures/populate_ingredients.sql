INSERT INTO recipes (name, serving_size, unit, protein, carbohydrate, fat, calories) VALUES ('Oatmeal with Milk and Banana', 400, 'g', 9.1, 55.7, 7.1, 313.0);

INSERT INTO ingredients (recipe_id, food_normalized_id, name, amount, unit, protein, carbohydrate, fat, calories) VALUES 
    (4, (SELECT id FROM foods_normalized WHERE name = 'oatmeal'), 'oatmeal', 50, 'g', 1.6, 6.2, 0.65, 35.5),
    (4, (SELECT id FROM foods_normalized WHERE name = 'milk'), 'milk', 200, 'g', 6.0, 9.0, 6.0, 120.0),
    (4, (SELECT id FROM foods_normalized WHERE name = 'banana'), 'banana', 150, 'g', 1.5, 40.5, 0.45, 157.5);