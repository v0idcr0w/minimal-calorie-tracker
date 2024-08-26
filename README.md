# Minimal Calorie and Macronutrient Tracker

As the name states, this is a very minimialistic calorie and macronutrient tracker. Features that are found in more modern trackers such as bar code scanner (MyFitnessPal) and algorithmic predictions (MacroTracker) are missing here, and it's a desktop app. 

### Features 
- The app tracks weight, daily macronutrients and calories consumed displaying results with charts and tables
- You can manually enter foods into a foods list, which becomes available to use in meals
- In the recipes tab, you can create custom recipes based on the foods available in your foods list
- In the meals tab, you create your meals for the day and enter the foods or recipes consumed and their amount; calculations are done automatically
- You can set a meal as constant if you want to keep it the same for the following days (this avoids repetition if you tend to eat the same foods every day)
- Your progress is automatically updated, and daily calories/macronutrients/current weight reset once the day ends
- In the Logs tab, you can see your progress in a tabular format and edit the contents of the other days, if for some reason you forgot to enter one of your meals into the app 

### The Tech Stack 
- Database: SQLite (the database is created when you first execute the app)
- Backend: Rust with <a href="https://tauri.app" title="tauri">Tauri</a> 
- Frontend: SvelteKit
- Styling: Tailwindcss  

### Credits
Huge thanks to the following sources, because the design of this app would be terrible without them: 
- <a href="https://www.flaticon.com/free-icons/pie-chart" title="pie chart icons">Pie chart icons created by DinosoftLabs - Flaticon</a> 
- Shadcn-Svelte (Component Library)
- Lucide Svelte (Icons Library)


### Updates TODO
- [x] Foods Tab
- [ ] Recipes Tab
- [ ] Meals Tab
- [ ] History Tab
- [ ] Logs Tab
- [ ] Charts Tab
- [ ] Home Tab 