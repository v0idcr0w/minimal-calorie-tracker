<script>
    import { invoke } from "@tauri-apps/api";
    import { onMount } from "svelte";
    import { foodsNormalized } from './store.js';
    import { toTitleCase } from './titleCase.js';
    import MaterialFloatingLabel from "./MaterialFloatingLabel.svelte";

    // props 
    export let recipe; 
    export let onDelete; 

    // ingredient variables
    let ingredients = []; 
    let editIngredientFlags = []; 
    let newIngredientAmounts = []; 

    // rename variables 
    let renameActive = false;
    let newRecipeName = ''; 
    
    // dropdown selection variables 
    let dropdownActive = false; 
    let selectedFoodId = 0; 
    let selectedFoodAmount = 0; 
    let selectedFood = {}; 

    //  editing serving size variables 
    let editServingSizeActive = false; 
    let newServingSize = 0; 
    let newUnit = recipe.unit; 

    onMount(async () => {
        await refreshIngredients(); 
    }); 

    async function refreshIngredients() {
        ingredients = await invoke('get_ingredients_by_recipe_id', { recipeId: recipe.id }); 
        editIngredientFlags = new Array(ingredients.length).fill(false);
        newIngredientAmounts = ingredients.map(ingredient => ingredient.amount); 
    }

    async function renameRecipe() {
        await invoke('update_recipe_name', { recipeId: recipe.id, newName: newRecipeName }); 
        // update the name here in the display to avoid calling the backend again 
        recipe.name = newRecipeName; 
        renameActive = false; 
        newRecipeName = '';
    }

    async function updateServingSize(newServingSize, newUnit) {
        recipe = await invoke('update_recipe_serving_size',  { recipe, newServingSize, newUnit }); 
        editServingSizeActive = false;
    }

    function handleSelect(event) {
        // handles selection in dropdown menu to add new ingredient 
        const selectedId = event.target.value; // value is a string 
        selectedFoodId = selectedId; 
        selectedFood = $foodsNormalized.find((item) => item.id == selectedId);
    }

    async function addIngredientToRecipe(selectedFood, selectedFoodAmount) {
        recipe = await invoke('add_ingredient_to_recipe', { foodNormalized: selectedFood, recipeId: recipe.id, amount: Number(selectedFoodAmount) });
        // refresh ui 
        dropdownActive = false; 
        refreshIngredients(); 
        selectedFoodAmount = 0; 
    }

    async function deleteIngredientFromRecipe(ingredient) {
        recipe = await invoke('delete_ingredient_from_recipe', { ingredient });
        // refresh the list of ingredients 
        refreshIngredients(); 
    }

    async function updateIngredientAmount(ingredient, newAmount) {
        // fetch new recipe with update macros 
        // update in the UI 
        recipe = await invoke('update_ingredient_amount', { ingredient, newAmount });
        ingredient.amount = newAmount; 
        // reset all flags to false 
        editIngredientFlags = new Array(ingredients.length).fill(false); 
    }


</script>

<!-- *************************** -->
<!-- *************************** -->
<!-- *************************** -->

<div class="block w-full tracking-tighter rounded-lg bg-white p-2 shadow-[0_2px_15px_-3px_rgba(0,0,0,0.07),0_10px_20px_-2px_rgba(0,0,0,0.04)]">

<h3 class="text-neutral-700 text-center text-xl m-4 font-bold" on:dblclick={() => renameActive = true} >
    {#if !renameActive}
        {toTitleCase(recipe.name)} 
    {/if}


<!-- Renaming a recipe -->
{#if renameActive}
    <input type="text" name="recipeName" placeholder="Recipe Name" bind:value={newRecipeName} class="w-1/2 text-center" />
    <!-- Accept changes button -->
    <button class="relative inline-flex items-center justify-center p-0.5 mb-2 me-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-purple-600 to-blue-500 group-hover:from-purple-600 group-hover:to-blue-500 hover:text-white focus:ring-4 focus:outline-none focus:ring-blue-300 disabled:cursor-not-allowed " on:click={renameRecipe} disabled={!newRecipeName}>
        <span class="relative transition-all ease-in duration-75 bg-white rounded-md group-hover:bg-opacity-0">
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="M382-240 154-468l57-57 171 171 367-367 57 57-424 424Z"/></svg>
            </span>
    </button>

    <!-- Reject changes button -->
    <button class="relative inline-flex items-center justify-center p-0.5 mb-2 me-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-purple-600 to-blue-500 group-hover:from-purple-600 group-hover:to-blue-500 hover:text-white focus:ring-4 focus:outline-none focus:ring-blue-300" on:click={() => renameActive = false }>
        <span class="relative transition-all ease-in duration-75 bg-white rounded-md group-hover:bg-opacity-0">
          <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor" ><path d="m256-200-56-56 224-224-224-224 56-56 224 224 224-224 56 56-224 224 224 224-56 56-224-224-224 224Z"/></svg>
        </span>
      </button> 
{/if}
</h3>  
<!-- Delete -->
<div class="flex justify-center">
<button class="icon-button mb-2 mx-2" on:click={onDelete}>
    <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="M280-120q-33 0-56.5-23.5T200-200v-520h-40v-80h200v-40h240v40h200v80h-40v520q0 33-23.5 56.5T680-120H280Zm400-600H280v520h400v-520ZM360-280h80v-360h-80v360Zm160 0h80v-360h-80v360ZM280-720v520-520Z"/></svg>
</button>
</div>

<table class="table-auto mx-auto w-1/2">
    <tr>
        <td class="text-center" colspan="2">Serving size</td>
        <td>
            <!-- Edit button -->
            <button class="relative inline-flex items-center justify-center p-0.5 mb-2 me-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-purple-600 to-blue-500 group-hover:from-purple-600 group-hover:to-blue-500 hover:text-white focus:ring-4 focus:outline-none focus:ring-blue-300 disabled:cursor-not-allowed " on:click={() => editServingSizeActive = !editServingSizeActive}>
                <span class="relative transition-all ease-in duration-75 bg-white rounded-md group-hover:bg-opacity-0">
                {#if !editServingSizeActive}
                <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="M200-200h57l391-391-57-57-391 391v57Zm-80 80v-170l528-527q12-11 26.5-17t30.5-6q16 0 31 6t26 18l55 56q12 11 17.5 26t5.5 30q0 16-5.5 30.5T817-647L290-120H120Zm640-584-56-56 56 56Zm-141 85-28-29 57 57-29-28Z"/></svg>
                {:else}
                <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor" ><path d="m256-200-56-56 224-224-224-224 56-56 224 224 224-224 56 56-224 224 224 224-56 56-224-224-224 224Z"/></svg>
                {/if}
            </span>
            </button>
        </td>
    </tr>
    <tr>
    {#if !editServingSizeActive}
        <td colspan="2" class="text-center"> {recipe.serving_size} 
         {recipe.unit} </td>
    {:else}
    <td class="text-center" colspan="2"> 
    <div class="flex">
    <input type="number" min=0 placeholder=0 bind:value={newServingSize} class="w-1/2 text-center" />  
    <input type="text" placeholder="unit" bind:value={newUnit} class="w-1/2 text-center" /> 
    </div>
    </td> 
    <td> <button class="relative inline-flex items-center justify-center p-0.5 mb-2 me-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-purple-600 to-blue-500 group-hover:from-purple-600 group-hover:to-blue-500 hover:text-white focus:ring-4 focus:outline-none focus:ring-blue-300 disabled:cursor-not-allowed" on:click={updateServingSize(newServingSize, newUnit)} disabled={!(newServingSize > 0 && newUnit)}>
        <span class="relative transition-all ease-in duration-75 bg-white rounded-md group-hover:bg-opacity-0">
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="M382-240 154-468l57-57 171 171 367-367 57 57-424 424Z"/></svg>
            </span>
    </button> </td>
    {/if}
    </tr>
</table>

<!-- Ingredient list -->


<ul class="w-full text-sm font-medium text-gray-900 bg-white border border-gray-200 rounded-lg">
    {#each ingredients as ingredient, index (ingredient.id)}

    <li class="w-full px-4 py-2 border-b border-gray-200 rounded-t-lg flex items-center text-left">
        <button class="relative inline-flex items-center p-0.5 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-purple-600 to-blue-500 group-hover:from-purple-600 group-hover:to-blue-500 hover:text-white focus:ring-4 focus:outline-none focus:ring-blue-300" on:click={() => deleteIngredientFromRecipe(ingredient)}>
            <span class="relative transition-all ease-in duration-75 bg-white rounded-md group-hover:bg-opacity-0">
                <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="M200-440v-80h560v80H200Z"/></svg>
            </span>
            </button> 


            <span class="inline-block mx-1 align-top">
                {toTitleCase(ingredient.name)}
            </span>

            
            {#if editIngredientFlags[index]}
            <input class="w-8 text-center inline-block my-2 align-top" type="number" min=0 placeholder=0 bind:value={newIngredientAmounts[index]} />
            {:else}
            <span class="inline-block my-2 align-top">
            {ingredient.amount}
            </span>
            {/if}

            <span class="inline-block my-2 mx-1 align-top">
            {ingredient.unit}
            </span>
    
            <button class="relative inline-flex items-center justify-center p-0.5 me-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-purple-600 to-blue-500 group-hover:from-purple-600 group-hover:to-blue-500 hover:text-white focus:ring-4 focus:outline-none focus:ring-blue-300" on:click={() => editIngredientFlags[index] = !editIngredientFlags[index]}>
                <span class="relative transition-all ease-in duration-75 bg-white rounded-md group-hover:bg-opacity-0">
                    {#if !editIngredientFlags[index]}
                    <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="M200-200h57l391-391-57-57-391 391v57Zm-80 80v-170l528-527q12-11 26.5-17t30.5-6q16 0 31 6t26 18l55 56q12 11 17.5 26t5.5 30q0 16-5.5 30.5T817-647L290-120H120Zm640-584-56-56 56 56Zm-141 85-28-29 57 57-29-28Z"/></svg>
                    {:else}
                    <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor" ><path d="m256-200-56-56 224-224-224-224 56-56 224 224 224-224 56 56-224 224 224 224-56 56-224-224-224 224Z"/></svg>
                    {/if}
                </span>
                </button>
                
                
        {#if editIngredientFlags[index]}
        <button class="relative inline-flex items-center justify-center p-0.5 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-purple-600 to-blue-500 group-hover:from-purple-600 group-hover:to-blue-500 hover:text-white focus:ring-4 focus:outline-none focus:ring-blue-300" on:click={updateIngredientAmount(ingredient, newIngredientAmounts[index])}>
            <span class="relative transition-all ease-in duration-75 bg-white rounded-md group-hover:bg-opacity-0">
                <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="M382-240 154-468l57-57 171 171 367-367 57 57-424 424Z"/></svg>
            </span>
            </button>
        {/if}
    </li>
    {/each}
</ul>

<!-- Adding new ingredient block -->
<div class="flex justify-center">
<button class="icon-button my-2" on:click={() => dropdownActive = !dropdownActive}>
    {#if !dropdownActive}
    <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="M440-440H200v-80h240v-240h80v240h240v80H520v240h-80v-240Z"/></svg>
    {:else}
    <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="m256-200-56-56 224-224-224-224 56-56 224 224 224-224 56 56-224 224 224 224-56 56-224-224-224 224Z"/></svg>
    {/if}
</button>


{#if dropdownActive && selectedFoodId}
<button class="icon-button my-2 mx-2" on:click={ addIngredientToRecipe(selectedFood, selectedFoodAmount) } disabled={!( selectedFoodAmount >= 0)}>
    <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="M382-240 154-468l57-57 171 171 367-367 57 57-424 424Z"/></svg>
</button>
{/if}
</div>

{#if dropdownActive}
    <div class="flex justify-center">
    <select class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 w-1/2 p-1" id="foodDropdown" on:change={handleSelect}>
<!-- first option is blank and unselectable -->
    <option value="" disabled selected hidden></option>
    {#each $foodsNormalized as foodNormalized (foodNormalized.id)}
    <option value={foodNormalized.id}>{toTitleCase(foodNormalized.name)}</option>
    {/each}
    </select>
    </div>
<!-- Select the amount -->
    {#if selectedFoodId}
    <div class="flex flex-col items-center mt-4">
        <div class="block text-sm">
    <MaterialFloatingLabel label="Amount ({selectedFood.unit})" bind:value={selectedFoodAmount} />
    </div>
    </div>
    {/if}

{/if}




<!-- Recipe totals information -->
<table class="mx-auto">
    <thead>
        <tr>
            <th colspan="3">Total</th>
        </tr>
    </thead>
    <tr>
        <td>Protein</td>
        <td>{recipe.protein.toFixed(1)}</td>
        <td>g</td>
    </tr>
    <tr>
        <td>Carbohydrates</td>
        <td>{recipe.carbohydrate.toFixed(1)}</td>
        <td>g</td>
    </tr>
    <tr>
        <td>Fats</td>
        <td>{recipe.fat.toFixed(1)}</td>
        <td>g</td>
    </tr>
    <tr>
        <td>Calories</td>
        <td>{recipe.calories.toFixed(0)}</td>
        <td>kcal</td>
    </tr>
</table>

</div>