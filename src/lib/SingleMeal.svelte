<script>
    // this part contains the logic related to rendering foods, editing and deleting a food from a given meal. 
    import { invoke } from "@tauri-apps/api";
    import { onMount } from "svelte";
    import { toTitleCase } from "./titleCase";
    import { foodsNormalized, recipes } from './store.js';
    import Dropdown from "./Dropdown.svelte";
    import GradientButton from "./GradientButton.svelte"; 
    import SvgOk from './SvgOk.svelte';
    import SvgEdit from './SvgEdit.svelte';
    import SvgCancel from './SvgCancel.svelte';
    import SvgAdd from './SvgAdd.svelte'; 
    import SvgRemove from './SvgRemove.svelte'; 

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
    <li class="w-full px-2 py-2 border-b border-gray-200 rounded-t-lg flex items-center justify-between text-left">
        <div class="-me-2 -mb-2">
        <GradientButton onClick={() => deleteFood(food)}>
            <SvgRemove /> 
        </GradientButton>
        </div>

        <div class="flex items-center">
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
        </div>

        <div class="-mb-2 flex items-center">
        {#if !editableArray[index]}
        
        <GradientButton onClick={() => editableArray[index] = !editableArray[index]}>
            <SvgEdit /> 
        </GradientButton>
        
        {:else}
        <GradientButton onClick={() => editableArray[index] = !editableArray[index]}>
            <SvgCancel />
        </GradientButton>
        {/if}
        
    
        <div class="-me-2">
        {#if editableArray[index]}
        <GradientButton onClick={() => updateFood(food, newAmountArray[index])} disabled={!(newAmountArray[index] >= 0)}>
            <SvgOk />
        </GradientButton>  
        {/if}
        </div>
        </div>

       </li> 
    {/each}
</ul>


<div class="flex items-center justify-center my-4">
<!-- Dropdown button for foods list -->
<button class="text-button w-32 p-1 mx-2" on:click={() => 
{dropdownActiveFood = !dropdownActiveFood;
 dropdownActiveRecipe = false} }>
    {#if !dropdownActiveFood}
    <SvgAdd /> food 
    {:else}
    <SvgCancel /> Cancel
    {/if}
</button>


<!-- Dropdown button for recipe list -->
<button class="text-button w-32 p-1 mx-2" on:click={() => 
{dropdownActiveRecipe = !dropdownActiveRecipe;
dropdownActiveFood = false} }>
    {#if !dropdownActiveRecipe}
    <SvgAdd /> recipe 
    {:else}
    <SvgCancel /> Cancel
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