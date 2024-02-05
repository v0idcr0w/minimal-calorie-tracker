<script>
    import { invoke } from "@tauri-apps/api";
    import { onMount } from "svelte";
    import { logId, foodsNormalized } from './store.js';
    import { toTitleCase } from './titleCase.js';

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
        recipe = await invoke('add_ingredient_to_recipe', { foodNormalized: selectedFood, recipeId: recipe.id, amount: selectedFoodAmount }); 
        dropdownActive = false; 
        refreshIngredients(); 
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


<h3>{toTitleCase(recipe.name)} <button on:click={onDelete}>Delete</button> </h3>  
<button on:click={() => renameActive = !renameActive}>{renameActive ? "Cancel" : "Rename"}</button>

<!-- Renaming a recipe -->
{#if renameActive}
    <input type="text" name="recipeName" placeholder="Recipe Name" bind:value={newRecipeName} />
    <button on:click={renameRecipe} disabled={!newRecipeName}>Ok</button>
{/if}

<p>
{#if editServingSizeActive}
    <input type="number" min=0 placeholder=0 bind:value={newServingSize} /> <input type="text" placeholder="unit" bind:value={newUnit} />
    <button on:click={updateServingSize(newServingSize, newUnit)} disabled={!(newServingSize > 0 && newUnit)}>Ok</button>
{:else}
    Serving size: {recipe.serving_size} {recipe.unit} 
{/if}

<!-- Editing Serving Size / Unit -->
<button on:click={() => editServingSizeActive = !editServingSizeActive}>{ !editServingSizeActive ? "Edit" : "Cancel" }</button>
</p>


<!-- Render ingredient list of the recipe -->
<table>
{#each ingredients as ingredient, index (ingredient.id)}
    <tr>
    <!-- Delete Ingredient Button -->
    <td>
        <button on:click={() => deleteIngredientFromRecipe(ingredient)}>-</button>
    </td>
    
    <td>{toTitleCase(ingredient.name)}</td>

    <td>
        {#if editIngredientFlags[index]}
        <input style="width: 50px;" type="number" min=0 placeholder=0 bind:value={newIngredientAmounts[index]} />
        {:else}
        {ingredient.amount}
        {/if}
    </td>
    <td>{ingredient.unit}</td>

    <!-- Edit Ingredient Amount Button -->
    <td>
        <button on:click={() => editIngredientFlags[index] = !editIngredientFlags[index]}> {!editIngredientFlags[index] ? "Edit" : "Cancel"} </button>
    </td>
    
    <!-- Edit confirmation button -->
    <td>
    {#if editIngredientFlags[index]}
        <button on:click={updateIngredientAmount(ingredient, newIngredientAmounts[index])}>Ok</button>
    {/if}
    </td>

    
    </tr>
{/each}
</table>


<!-- Adding new ingredient block -->
<button on:click={() => dropdownActive = !dropdownActive}>{!dropdownActive ? "+" : "Cancel"}</button>
{#if dropdownActive}
    <select id="foodDropdown" on:change={handleSelect}>
<!-- first option is blank and unselectable -->
    <option value="" disabled selected hidden></option>
    {#each $foodsNormalized as foodNormalized (foodNormalized.id)}
    <option value={foodNormalized.id}>{toTitleCase(foodNormalized.name)}</option>
    {/each}
    </select>

<!-- Select the amount -->
    {#if selectedFoodId}
    <input type="number" min=0 placeholder=0 bind:value={selectedFoodAmount} /> {selectedFood.unit}
    {/if}
<!-- Confirm adding a new ingredient -->
<button on:click={ addIngredientToRecipe(selectedFood, selectedFoodAmount) } disabled={!(selectedFoodAmount >= 0)}>Ok</button>
{/if}



<!-- Recipe totals information -->
<table>
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

