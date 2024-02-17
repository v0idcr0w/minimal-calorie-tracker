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
- [OK] Goals: missing fetch from database 
- [OK] Delete confirmation
- [OK] Sort foods, recipes etc by the id in descending order 
- [OK] Option to import foods from CSV file. 
- [OK] Option to export foods to CSV file. 
- [OK] Install charts.js 
- [OK] Pizza chart showing your current macro split for the day in the meals page; also in the home page for your goals) 
- [OK] Double Line chart showing your weight progress; and your calories progress over the weeks (only if there is more than 1 data point). https://github.com/SauravKanchan/svelte-chartjs 
- [OK] Compare the users goals with their current progress in the Daily Log page (+ 0.5 kcal relative to goal etc...) 

### TODO: UI BASICS
- [OK] Foods (fix button sizes and input boxes for new food)
- [OK] Recipes (copy the style of foods)
- [OK] Meals (copy the style of foods)
- Home 
- [OK] Logs
- [OK] Progress Charts
- [OK] Navigation bar 

### TODO: FIXES/OTHER STUFF
- Fix removing id when the csv file is saved 
- Menu to display chart data only for the last X days. 
- Tooltips/help
- Change weight goal instead of being lose to a target 

https://tw-elements.com/