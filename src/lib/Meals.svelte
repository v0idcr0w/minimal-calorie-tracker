<script>
    import { invoke } from '@tauri-apps/api/tauri'
    import { onMount, onDestroy } from 'svelte'
    import { toTitleCase } from './titleCase.js';
    import { dailyTotals, today, logId } from './store.js';
    import Dropdown from './Dropdown.svelte';
    import SingleMeal from './SingleMeal.svelte'; 

    const todayFormatted = $today.toISOString().split('T')[0];
    let meals = []; 
    let newMeal = {}; 
    let createMealActive = false; 
    let dropdownsActive = []; 
    dailyTotals.set({calories: 0, protein: 0, carbohydrate: 0, fat: 0}); 
    let mealIds = []; 

    async function refreshMeals() {
        // later on this will be get meals by date 
        meals = await invoke('get_meals');
        meals.sort((a, b) => new Date(b.entry_timestamp) - new Date(a.entry_timestamp));
        dropdownsActive = Array.from({ length: meals.length }, () => false);
        newMeal = {id: 0, log_id: $logId, name: '', entry_timestamp: ''};
        mealIds = meals.map((obj) => obj.id); 
    }

    async function updateTotals() {
        dailyTotals.set(await invoke('compute_daily_macros', { mealIds }));
        await invoke('update_log_totals', { logId: $logId, dailyTotals: $dailyTotals })
    }

    async function addNewMeal(newMeal) {
        // to make it compatible with Rust NaiveDateTime, remove the last character that represents the timezone. 
        newMeal.entry_timestamp = $today.toISOString().slice(0, 23); 
        await invoke('add_new_meal', { newMeal })
        await refreshMeals(); 
        await updateTotals(); 
        createMealActive = false;
    }

    async function deleteMeal(meal) {
        await invoke('delete_meal', { meal });
        await refreshMeals();
        await updateTotals(); 
    }

    onMount(async () => {
      await refreshMeals(); 
      dailyTotals.set(await invoke('compute_daily_macros', { mealIds }));
      window.addEventListener('foodAdded', updateTotals); 
      window.addEventListener('foodModified', updateTotals); 
    });

    onDestroy(() => {
        window.removeEventListener('foodAdded', updateTotals);
        window.removeEventListener('foodModified', updateTotals); 
    })


</script>

<h3>Calorie and macronutrient consumption on {todayFormatted}</h3>
<ul>
    <li>Calories: {$dailyTotals.calories.toFixed(1)} kcal </li>
    <li>Protein: {$dailyTotals.protein.toFixed(1)} g</li>
    <li>Carbohydrates: {$dailyTotals.carbohydrate.toFixed(1)} g</li>
    <li>Fats: {$dailyTotals.fat.toFixed(1)} g</li>
</ul>
<button on:click={() => createMealActive = !createMealActive}>{ createMealActive ? "Cancel" : "Create Meal" }</button>

{#if createMealActive}
<input name="type" placeholder="Meal type" bind:value={newMeal.name} />
<button on:click={addNewMeal(newMeal)}>Confirm</button>
{/if}
<br />

{#each meals as meal, index (meal.id) } 
<b>{toTitleCase(meal.name)}</b>  
<button on:click={deleteMeal(meal)}>Delete</button>

<br />
<!-- clicking this button expands the dropdown list -->
<button on:click={() => dropdownsActive[index] = !dropdownsActive[index]}> {dropdownsActive[index] ? "-" : "+"} </button>
{#if dropdownsActive[index]}
<Dropdown mealId={meal.id}/>
{/if}
<!-- render all foods associated with this meal -->
<SingleMeal mealId={meal.id} />


{/each} 