<script>
    // this part contains the logic related to rendering foods, editing and deleting a food from a given meal. 
    import { invoke } from "@tauri-apps/api";
    import { onMount } from "svelte";
    import { toTitleCase } from "./titleCase";
    import { foodsNormalized, recipes } from './store.js';
    import Dropdown from "./Dropdown.svelte";

    // props 
    export let mealId; 
    export let onUpdate; 

    // arrays to store food information 
    let foods = []; 
    let editableArray = []; 
    let newAmountArray = [];  
    let computedMacros = {}; 

    // button control
    let dropdownActiveFood = false; 
    let dropdownActiveRecipe = false; 

    async function refreshFoods() {
        foods = await invoke('get_foods_by_meal_id', { mealId }); 
        computedMacros = await invoke('compute_meal_macros', { mealId }); 
        editableArray = new Array(foods.length).fill(false);
        newAmountArray = foods.map((food) => food.amount);
    }

    onMount(async () => {
        if ($foodsNormalized.length === 0) {
            foodsNormalized.set(await invoke('get_foods_normalized')); 
        }
        if ($recipes.length === 0) {
            recipes.set(await invoke('get_all_recipes'));
        }
        await refreshFoods();  
    }); 

    async function deleteFood(food) {
        await invoke('delete_food', { food });
        await refreshFoods();  
        await onUpdate(); 
    }

    async function updateFood(food, newAmount) {
        await invoke('update_food', { food, newAmount }); 
        await refreshFoods(); 
        await onUpdate(); 
    }

    async function addNewFood(selectedId, amount) {
        await invoke('add_new_food', { selectedId, amount, mealId })
        // dispatch this event when a new food gets added
        await refreshFoods();  
        await onUpdate(); 
        dropdownActiveFood = false; 
    }

    async function addNewRecipe(selectedId, amount) {
        await invoke('add_new_food_from_recipe', { selectedId, amount, mealId })
        await refreshFoods();  
        await onUpdate(); 
        dropdownActiveRecipe = false; 
    }
</script>

<div>
    <ul class="w-full text-sm font-medium text-gray-900 bg-white border border-gray-200 rounded-lg">    
    {#each foods as food, index (food.id)}
    <li class="w-full px-2 py-2 border-b border-gray-200 rounded-t-lg flex items-center text-left">
        <button class="relative inline-flex items-center p-0.5 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-purple-600 to-blue-500 group-hover:from-purple-600 group-hover:to-blue-500 hover:text-white focus:ring-4 focus:outline-none focus:ring-blue-300" on:click={deleteFood(food)}>
            <span class="relative transition-all ease-in duration-75 bg-white rounded-md group-hover:bg-opacity-0">
                <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="M200-440v-80h560v80H200Z"/></svg>
            </span>
        </button>
        <span class="inline-block mx-1 align-top">
            {toTitleCase(food.name)}
        </span>

        {#if !editableArray[index]}
        <span class="inline-block my-2 align-top">{food.amount}</span>
        {:else}
        <input type="number" min=0 bind:value={newAmountArray[index]} placeholder={food.amount} class="w-8 text-center inline-block my-2 align-top"  />
        {/if}

        <span class="inline-block my-2 mx-1 align-top">
            {food.unit}
        </span>

        <button class="relative inline-flex items-center justify-center p-0.5 me-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-purple-600 to-blue-500 group-hover:from-purple-600 group-hover:to-blue-500 hover:text-white focus:ring-4 focus:outline-none focus:ring-blue-300" on:click={() => editableArray[index] = !editableArray[index] }>
            <span class="relative transition-all ease-in duration-75 bg-white rounded-md group-hover:bg-opacity-0">
            {#if !editableArray[index]}
            
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="M200-200h57l391-391-57-57-391 391v57Zm-80 80v-170l528-527q12-11 26.5-17t30.5-6q16 0 31 6t26 18l55 56q12 11 17.5 26t5.5 30q0 16-5.5 30.5T817-647L290-120H120Zm640-584-56-56 56 56Zm-141 85-28-29 57 57-29-28Z"/></svg>
            {:else} 
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor" ><path d="m256-200-56-56 224-224-224-224 56-56 224 224 224-224 56 56-224 224 224 224-56 56-224-224-224 224Z"/></svg>
            {/if}
        </span>
        </button>

        {#if editableArray[index]}
        <button class="relative inline-flex items-center justify-center p-0.5 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-purple-600 to-blue-500 group-hover:from-purple-600 group-hover:to-blue-500 hover:text-white focus:ring-4 focus:outline-none focus:ring-blue-300" on:click={updateFood(food, newAmountArray[index])} disabled={!(newAmountArray[index] >= 0)} >
            <span class="relative transition-all ease-in duration-75 bg-white rounded-md group-hover:bg-opacity-0">
                <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="M382-240 154-468l57-57 171 171 367-367 57 57-424 424Z"/></svg>
            </span>
        </button>
        {/if}

       </li> 
    {/each}
</ul>


<div class="flex items-center justify-center my-4">
<!-- Dropdown button for foods list -->
<button class="text-button w-32 p-1 mx-2" on:click={() => 
{dropdownActiveFood = !dropdownActiveFood;
 dropdownActiveRecipe = false} }>
    {#if !dropdownActiveFood}
    <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="M440-440H200v-80h240v-240h80v240h240v80H520v240h-80v-240Z"/></svg> Add food 
    {:else}
    <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="m256-200-56-56 224-224-224-224 56-56 224 224 224-224 56 56-224 224 224 224-56 56-224-224-224 224Z"/></svg> Cancel
    {/if}
</button>


<!-- Dropdown button for recipe list -->
<button class="text-button w-32 p-1 mx-2" on:click={() => 
{dropdownActiveRecipe = !dropdownActiveRecipe;
dropdownActiveFood = false} }>
    {#if !dropdownActiveRecipe}
    <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="M440-440H200v-80h240v-240h80v240h240v80H520v240h-80v-240Z"/></svg> Add recipe 
    {:else}
    <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="m256-200-56-56 224-224-224-224 56-56 224 224 224-224 56 56-224 224 224 224-56 56-224-224-224 224Z"/></svg> Cancel
    {/if}
</button>
</div>


    {#if dropdownActiveFood}
        <Dropdown onAdd={addNewFood} options={$foodsNormalized} />
    {/if}

    {#if dropdownActiveRecipe}
        <Dropdown onAdd={addNewRecipe} options={$recipes} />
    {/if}


<!-- Table containing the total amount of macros for the meal -->
{#if Object.keys(computedMacros).length !== 0}
<table class="mx-auto text-sm">
    <thead>
        <tr>
            <th colspan="3">Meal totals</th>
        </tr>
    </thead>
    <tr>
        <td>Calories</td>
        <td>{computedMacros.calories.toFixed(1)}</td>
        <td>kcal</td>
    </tr>
    <tr>
        <td>Protein</td>
        <td>{computedMacros.protein.toFixed(1)}</td>
        <td>g</td>
    </tr>
    <tr>
        <td>Carbohydrates</td>
        <td>{computedMacros.carbohydrate.toFixed(1)}</td>
        <td>g</td>
    </tr>
    <tr>
        <td>Fats</td>
        <td>{computedMacros.fat.toFixed(1)}</td>
        <td>g</td>
    </tr>
</table>

{/if}

</div>