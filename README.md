# Minimal Calorie and Macronutrient Tracker

As the name states, this is a very minimalistic calorie and macronutrient tracker. Features that are found in more modern trackers such as bar code scanner (MyFitnessPal) and algorithmic predictions (MacroTracker) are missing here, and it's a desktop app. 

If you enjoy the app please consider supporting me:

<a href="https://buymeacoffee.com/williamqprh" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: 41px !important;width: 174px !important;box-shadow: 0px 3px 2px 0px rgba(190, 190, 190, 0.5) !important;-webkit-box-shadow: 0px 3px 2px 0px rgba(190, 190, 190, 0.5) !important;" ></a>

### Features 
- The app tracks weight, daily macronutrients and calories consumed displaying results with charts and tables
- You can manually enter foods into a foods list, which becomes available to use in meals
- In the recipes tab, you can create custom recipes based on the foods available in your foods list
- In the meals tab, you create your meals for the day and enter the foods or recipes consumed and their amount; calculations are done automatically
- You can set a meal as constant if you want to keep it the same for the following days (this avoids repetition if you tend to eat the same foods every day)
- Your progress is automatically updated, and daily calories/macronutrients/current weight reset once the day ends
- In the Logs tab, you can see your progress in a tabular format and edit the contents of the other days, if for some reason you forgot to enter one of your meals into the app

### Differences vs. typical calorie trackers 
- No need for an internet connection (works offline) 
- Currently desktop only
- No bar code scanner (it is a premium feature in most apps)
- No algorithmic predictions (you must decide for yourself how to alter your caloric intake based on how your weight is trending) 
- No micronutrient/fiber tracking 
- No water tracking (largely unnecessary, just drink as much as you need to)

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
