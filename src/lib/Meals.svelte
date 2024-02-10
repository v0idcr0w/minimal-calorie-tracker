<script>
    import { invoke } from '@tauri-apps/api/tauri'
    import { confirm } from '@tauri-apps/api/dialog';
    import { onMount } from 'svelte'
    import { toTitleCase } from './titleCase.js';
    import { dailyTotals, today, logId } from './store.js'; 
    import SingleMeal from './SingleMeal.svelte'; 
    import MacrosPie from './MacrosPie.svelte'; 

    const todayFormatted = $today.toISOString().split('T')[0];

    dailyTotals.set({calories: 0, protein: 0, carbohydrate: 0, fat: 0}); 
    $: totalMacros = $dailyTotals.protein + $dailyTotals.carbohydrate + $dailyTotals.fat; 
    $: macros = [$dailyTotals.protein / totalMacros * 100, $dailyTotals.carbohydrate / totalMacros * 100, $dailyTotals.fat / totalMacros * 100]; 
    
    let meals = []; 
    let newMeal = {}; 
    let mealIds = []; 
    let createMealActive = false; 

    async function refreshMeals() {
        meals = await invoke('get_meals_by_log_id', { logId: $logId });
        meals.sort((a, b) => new Date(b.entry_timestamp) - new Date(a.entry_timestamp));
        mealIds = meals.map((obj) => obj.id); 
        newMeal = {id: 0, log_id: $logId, name: '', entry_timestamp: ''};
    }

    async function updateTotals() {
        dailyTotals.set(await invoke('compute_daily_macros', { mealIds }));
        await invoke('update_log_totals', { logId: $logId, dailyTotals: $dailyTotals }); 
    }

    async function addNewMeal(newMeal) {
        // to make it compatible with Rust NaiveDateTime, remove the last character that represents the timezone. 
        newMeal.entry_timestamp = $today.toISOString().slice(0, 23); 
        await invoke('add_new_meal', { newMeal })
        await refreshMeals(); 
        // reset the newMeal object 
        newMeal = {id: 0, log_id: $logId, name: '', entry_timestamp: ''};
        // set the createMealActive to false to hide the input field
        createMealActive = false;
        // calling updateTotals is unnecessary because meals are initialized empty 
    }

    async function deleteMeal(meal) {
        const confirmed = await confirm('This action cannot be reverted. Are you sure?', { title: 'Confirm', type: 'info' });
        if (!confirmed) return;
        await invoke('delete_meal', { meal });
        await refreshMeals();
        await updateTotals(); 
    }

    onMount(async () => {
        if (!$logId) {
            logId.set(await invoke('get_todays_log').id);
        }
        await refreshMeals(); 
        await updateTotals();
    });

</script>

<!-- Total meals in Chart format -->
<MacrosPie {macros} />

<!-- Total calories and macronutrients (sum of all meals) -->
<table>
    <thead>
        <tr>
            <th colspan="3">Total for {todayFormatted}</th>
        </tr>
    </thead>
    <tr>
        <td>Calories</td>
        <td>{$dailyTotals.calories.toFixed(1)}</td>
        <td>kcal</td>
    </tr>
    <tr>
        <td>Protein</td>
        <td>{$dailyTotals.protein.toFixed(1)}</td>
        <td>g</td>
    </tr>
    <tr>
        <td>Carbohydrates</td>
        <td>{$dailyTotals.carbohydrate.toFixed(1)}</td>
        <td>g</td>
    </tr>
    <tr>
        <td>Fats</td>
        <td>{$dailyTotals.fat.toFixed(1)}</td>
        <td>g</td>
    </tr>
</table>


<!-- Meal creation -->
<button on:click={() => createMealActive = !createMealActive}>{ createMealActive ? "Cancel" : "Create Meal" }</button>

{#if createMealActive}
<input name="type" placeholder="Meal type" bind:value={newMeal.name} />
<button on:click={addNewMeal(newMeal)}>Confirm</button>
{/if}
<br />

{#each meals as meal, index (meal.id) } 

<!-- Delete meal -->
<h3>{toTitleCase(meal.name)} 
<button on:click={deleteMeal(meal)}>Delete</button>
</h3> 

<!-- render all foods associated with this meal -->
<SingleMeal mealId={meal.id} onUpdate={updateTotals} />



{/each} 