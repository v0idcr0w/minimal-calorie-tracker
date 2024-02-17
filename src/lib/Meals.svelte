<script>
    import { invoke } from '@tauri-apps/api/tauri'
    import { confirm } from '@tauri-apps/api/dialog';
    import { onMount } from 'svelte'
    import { toTitleCase } from './titleCase.js';
    import { dailyTotals, today, logId } from './store.js'; 
    import { base } from '$app/paths';
    import SingleMeal from './SingleMeal.svelte'; 
    import MacrosPie from './MacrosPie.svelte'; 
	import MaterialFloatingLabel from './MaterialFloatingLabel.svelte';

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
<!-- {#if meals.length > 0} 
    <MacrosPie {macros} />
{/if} -->

<div class="mx-4">

<div class="flex flex-col items-center">
<!-- Meal creation -->
    <div class="mb-4">
    <button class="text-button" on:click={() => createMealActive = !createMealActive}>
        {#if !createMealActive }
        <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="M440-440H200v-80h240v-240h80v240h240v80H520v240h-80v-240Z"/></svg> New meal 
        {:else}
        <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="m256-200-56-56 224-224-224-224 56-56 224 224 224-224 56 56-224 224 224 224-56 56-224-224-224 224Z"/></svg> Cancel
        {/if}
    </button>
    </div>

{#if createMealActive}
<MaterialFloatingLabel label="Meal type" bind:value={newMeal.name} />
    <div class="mb-4">
    <button class="text-button" on:click={addNewMeal(newMeal)}>
        <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="M382-240 154-468l57-57 171 171 367-367 57 57-424 424Z"/></svg> OK
    </button>
    </div>
{/if}
</div>

<table class="mx-auto tracking-tighter text-sm mb-2">
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

<div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
    {#each meals as meal, index (meal.id) } 
    <div class="block w-full text-center tracking-tighter rounded-lg bg-white p-2 shadow-[0_2px_15px_-3px_rgba(0,0,0,0.07),0_10px_20px_-2px_rgba(0,0,0,0.04)]">
<!-- Delete meal -->
    <h3 class="text-neutral-700 text-xl m-4 font-bold">{toTitleCase(meal.name)} 
    </h3> 
    <button class="icon-button mb-4" on:click={deleteMeal(meal)}>
        <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="M280-120q-33 0-56.5-23.5T200-200v-520h-40v-80h200v-40h240v40h200v80h-40v520q0 33-23.5 56.5T680-120H280Zm400-600H280v520h400v-520ZM360-280h80v-360h-80v360Zm160 0h80v-360h-80v360ZM280-720v520-520Z"/></svg>
    </button>

<!-- render all foods associated with this meal -->
    <SingleMeal mealId={meal.id} onUpdate={updateTotals} />
    </div>
{/each} 
</div>

</div>