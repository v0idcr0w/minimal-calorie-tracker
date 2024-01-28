<script>
    // this part contains the logic related to rendering foods, editing and deleting a food from a given meal. 
    import { invoke } from "@tauri-apps/api";
    import { onMount, onDestroy } from "svelte";
    import { toTitleCase } from "./titleCase";
    export let mealId; 

    let foods = []; 
    let editableArray = []; 
    let newAmount;  
    let validationError = ""; 
    let computedMacros = {}; 

    async function refreshFoods() {
        foods = await invoke('get_foods_by_meal_id', { mealId }); 
        computedMacros = await invoke('compute_meal_macros', { mealId }); 
        editableArray = Array.from({ length: foods.length }, () => false);
        
    }

    onMount(async () => {
        await refreshFoods(); 
        window.addEventListener('foodAdded', refreshFoods); 
    }); 

    onDestroy(() => {
        window.removeEventListener('foodAdded', refreshFoods);
    }); 

    async function deleteFood(food) {
        await invoke('delete_food', { food });
        await refreshFoods();  
    }

    async function updateFood(food, newAmount) {
        if (newAmount >= 0) {
            await invoke('update_food', { food, newAmount })
            await refreshFoods(); 
        } else {
            validationError = "Amount must be greater than 0"; 
        }
    }
</script>

<ul>
{#each foods as food, index (food.id)} 
<li>
    <b>{toTitleCase(food.name)}</b>
    <button on:click={() => editableArray[index] = !editableArray[index] }>{ editableArray[index] ? "Cancel" : "Edit"}</button>

    <button on:click={deleteFood(food)}>Delete</button>

    <ul>
        <li>
        {#if !editableArray[index]}
        {food.amount} {food.unit}
        {:else}
        <input type="number" min=0 bind:value={newAmount} placeholder={food.amount} style="width: 3em" /> {food.unit} 
        <button on:click={updateFood(food, newAmount)}>Ok</button>

        {#if validationError}
        <span class="error">{validationError}</span>
        {/if}

        {/if}


        </li>
        <li>{food.calories.toFixed(1)} kcal</li>   
    </ul> 
</li>
{/each}
</ul>
{#if Object.keys(computedMacros).length !== 0}
Totals: 
<ul>
    <li>
        Calories: {computedMacros.calories.toFixed(1)} kcal
    </li>
    <li>Protein: {computedMacros.protein.toFixed(1)} g</li>
    <li>
        Carbohydrates: {computedMacros.carbohydrate.toFixed(1)} g 
    </li>
    <li>Fats: {computedMacros.fat.toFixed(1)} g </li>
</ul>
{/if}