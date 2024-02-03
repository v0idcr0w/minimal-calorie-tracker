# Minimal Calorie and Macronutrient Tracker

### TODO: DATABASE
- [OK] Restructure the daily logger model accordingly 
- [OK] Change the meals table to include the date 
- [OK] Restructure the meals and food model accordingly 
- [OK] The many-to-many relationship between meals and foods is unnecessary -> REMOVED, changed to one-to-many between meals and food 
- [OK] There should be a one-to-many relationship between the meals and a single daily log  
- [OK]Include code to edit the daily logger macros and calories 
- [OK] Change the methods under foods, meals and daily loggers

### TODO: BASICS
- [OK] Daily Logs page should have a button to insert your daily weight for the day  
- [OK] When the total daily totals change, this change should be reflected in the database as well (do this inside the compute_daily_totals function to avoid multiple calls to the backend). 
- [OK] One daily log is automatically created for today when the user opens the home page, but only if one does not already exist.
- Meals should be fetched by their current date. 

### TODO: FEATURES
- Recipes: a recipe is built from a combination of foods from the foods database, like a meal, but you can also query for it when creating a new meal. This table should have no relationship to food_normalized but it should use the data from it to be created.  
- Option to import foods from CSV file
- Option to export foods to CSV file 
- Goals page -> lose/gain weight; rate of loss/gain per week and a percentage of recommendation; calories goal; macros goal (user can specify it - or this is automatically selected according to the currently available evidence); 

