# Minimal Calorie and Macronutrient Tracker

### TODO: DATABASE
- [OK] Restructure the daily logger model accordingly 
- [OK] Change the meals table to include the date 
- [OK] Restructure the meals and food model accordingly 
- [OK] The many-to-many relationship between meals and foods is unnecessary -> REMOVED, changed to one-to-many between meals and food 
- [OK] There should be a one-to-many relationship between the meals and a single daily log  
- [OK] Include code to edit the daily logger macros and calories 
- [OK] Change the methods under foods, meals and daily loggers

### TODO: BASICS
- [OK] Daily Logs page should have a button to insert your daily weight for the day  
- [OK] When the total daily totals change, this change should be reflected in the database as well 
- [OK] One daily log is automatically created for today when the user opens the home page, but only if one does not already exist.
- [OK] Meals should be fetched and displayed by their current date (not all meals!), or by the logId. New meals created should come with the timestamp.  
- [OK] Redesign the "Foods" tab 

### TODO: FEATURES
- [OK] Recipes: a recipe is built from a combination of foods from the foods database, like a meal, but you can also query for it when creating a new meal. This table should have no relationship to food_normalized but it should use the data from it to be created.  
- [OK] Enable the possibility of adding recipe under meals. 
- Goals: missing fetch from database (also, macronutrients should be automatically calculated based on 2 other macronutrient amount or percentage provided). Protein should be provided as g per kg of bodyweight by default. 
- Option to import foods & recipes from CSV file. 
- Option to export foods & recipes to CSV file. 
- Graphs (pizza chart showing your current macro split for the day); line chart showing your weight progress; and your calories progress (only if there is more than 1 data point). 
- Compare the users goals with their current progress. 

