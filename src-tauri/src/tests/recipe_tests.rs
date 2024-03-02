// use sqlx::SqlitePool;

// use crate::models::{food_normalized::FoodNormalized, ingredient::Ingredient, macros_total::MacrosTotal, recipe::Recipe}; 

// #[sqlx::test(fixtures("init", "populate_normalized", "populate_meals", "populate_foods", "create_recipes_ingredients_tables"))] 
// async fn test_create_empty_recipe(pool: SqlitePool) -> sqlx::Result<()> {
//     let mut empty_recipe = Recipe::new(0, "empty recipe".to_string(), 0.0, "g".to_string(), 0.0, 0.0, 0.0, 0.0);
//     empty_recipe.create_entry(&pool).await?; 

//     let empty_from_db = Recipe::get_by_id(empty_recipe.id, &pool).await?;

//     assert_eq!(empty_recipe, empty_from_db);

//     Ok(())
// }

// #[sqlx::test(fixtures("init", "populate_normalized", "populate_meals", "populate_foods", "create_recipes_ingredients_tables", "populate_recipes"))] 
// async fn test_delete_recipe(pool: SqlitePool) -> sqlx::Result<()> {
//     let total_recipes_before = Recipe::get_all(&pool).await?.len();

//     let to_delete = Recipe::get_by_id(1, &pool).await?; 
//     to_delete.delete_entry(&pool).await?; 
    
//     let total_recipes_after = Recipe::get_all(&pool).await?.len();

//     assert_eq!(total_recipes_before, total_recipes_after + 1);

//     Ok(())
// }

// #[sqlx::test(fixtures("init", "populate_normalized", "populate_meals", "populate_foods", "create_recipes_ingredients_tables", "populate_recipes"))] 
// async fn test_update_recipe_name(pool: SqlitePool) -> sqlx::Result<()> {
//     let mut to_change = Recipe::get_by_id(1, &pool).await?;
//     to_change.update_name("new name".to_string(), &pool).await?; 

//     let name_from_db = Recipe::get_by_id(1, &pool).await?.name; 

//     assert_eq!(name_from_db, "new name".to_string()); 

//     Ok(())
// }

// #[sqlx::test(fixtures("init", "populate_normalized", "populate_meals", "populate_foods", "create_recipes_ingredients_tables", "populate_recipes"))] 
// async fn test_update_recipe_serving_size(pool: SqlitePool) -> sqlx::Result<()> {
//     // this should reflect an update in the recipe's macros 
//     let mut recipe = Recipe::get_by_id(2, &pool).await?;
//     println!("Recipe before = {:?}", recipe);
//     recipe.update_serving_size(1.0, recipe.unit.clone()); 
//     recipe.update_entry(&pool).await?; 


//     let updated_recipe = Recipe::get_by_id(2, &pool).await?; 
//     assert_eq!(recipe, updated_recipe); 

//     println!("Recipe after = {:?}", recipe);


//     Ok(())
// }

// #[sqlx::test(fixtures("init", "populate_normalized", "populate_meals", "populate_foods", "create_recipes_ingredients_tables", "populate_recipes"))] 
// async fn test_create_ingredient(pool: SqlitePool)  -> sqlx::Result<()> {
//     // when a new ingredient is added, the recipe's macros should be updated. 

//     // start with an empty recipe 
//     let mut empty_recipe = Recipe::new(0, "fruit salad".to_string(), 0.0, "g".to_string(), 0.0, 0.0, 0.0, 0.0);
//     empty_recipe.create_entry(&pool).await?; 
//     println!("Recipe before = {:?}", empty_recipe);

//     // add a new ingredient to the recipe 
//     let food = FoodNormalized::get_by_id(1, &pool).await?; 
//     let mut ingredient = Ingredient::from(food, empty_recipe.id, 150.0); 
//     ingredient.create_entry(&pool).await?;

//     println!("Ingredient = {:?}", ingredient);

//     let macros_to_add = MacrosTotal::new(ingredient.protein, ingredient.carbohydrate, ingredient.fat, ingredient.calories); 

//     empty_recipe.update_macros(macros_to_add);
//     empty_recipe.update_entry(&pool).await?;

//     let updated_recipe = Recipe::get_by_id(empty_recipe.id, &pool).await?; 

//     println!("Recipe after = {:?}", updated_recipe);

//     assert_eq!(empty_recipe, updated_recipe);
//     Ok(())
// }


// #[sqlx::test(fixtures("init", "populate_normalized", "populate_meals", "populate_foods", "create_recipes_ingredients_tables", "populate_recipes", "populate_ingredients"))] 
// async fn test_delete_ingredient(pool: SqlitePool) -> sqlx::Result<()> {
//     // when an ingredient is deleted, the recipe's macros should be updated. 

//     let mut sample_recipe = Recipe::get_by_id(4, &pool).await?;

//     println!("Recipe before = {:?}", sample_recipe); 

//     let ingredients_before = Ingredient::get_by_recipe_id(sample_recipe.id, &pool).await?; 
//     let ingredient_to_delete = ingredients_before[0].clone(); 
//     ingredient_to_delete.delete_entry(&pool).await?; 

//     let macros_to_subtract = MacrosTotal::new(-ingredient_to_delete.protein, -ingredient_to_delete.carbohydrate, -ingredient_to_delete.fat, -ingredient_to_delete.calories);
//     sample_recipe.update_macros(macros_to_subtract);
//     sample_recipe.update_entry(&pool).await?;

//     println!("Recipe after = {:?}", sample_recipe); 

//     let updated_recipe = Recipe::get_by_id(sample_recipe.id, &pool).await?;

//     assert_eq!(sample_recipe, updated_recipe); 

//     Ok(())
// }

// #[sqlx::test(fixtures("init", "populate_normalized", "populate_meals", "populate_foods", "create_recipes_ingredients_tables", "populate_recipes", "populate_ingredients"))] 
// async fn test_delete_recipe_with_ingredients(pool: SqlitePool) -> sqlx::Result<()> {
//     // when a recipe is deleted, all of its associated ingredients should be deleted as well. 
//     let mut sample_recipe = Recipe::get_by_id(4, &pool).await?;

//     let ingredients_before_len = Ingredient::get_all(&pool).await?.len(); 

//     sample_recipe.delete_entry(&pool).await?; 

//     let ingredients_after_len = Ingredient::get_all(&pool).await?.len(); 
    
//     assert_eq!(ingredients_before_len, ingredients_after_len + 3);

//     Ok(())
// }

// #[sqlx::test(fixtures("init", "populate_normalized", "populate_meals", "populate_foods", "create_recipes_ingredients_tables", "populate_recipes", "populate_ingredients"))] 
// async fn test_update_ingredient_amount(pool: SqlitePool) -> sqlx::Result<()> {
//     // when an ingredient is updated, the recipe's macros should be updated. 

//     let mut ingredient = Ingredient::get_by_id(1, &pool).await?; 
//     let initial_macros = ingredient.into_macros_total(); 
    
//     println!("Macros before = {:?}", initial_macros);

//     ingredient.update(200.0); 
//     ingredient.update_entry(&pool).await?; 

//     let final_macros = ingredient.into_macros_total(); 
//     println!("Macros after = {:?}", final_macros); 
//     let delta_macros = final_macros - initial_macros; 

//     let mut recipe = Recipe::get_by_id(ingredient.recipe_id, &pool).await?;
//     println!("Recipe before = {:?}", recipe); 
//     recipe.update_macros(delta_macros); 
//     recipe.update_entry(&pool).await?;
//     println!("Recipe after = {:?}", recipe); 

//     let recipe_from_db = Recipe::get_by_id(ingredient.recipe_id, &pool).await?; 

//     assert_eq!(recipe, recipe_from_db);


//     Ok(())
// }

