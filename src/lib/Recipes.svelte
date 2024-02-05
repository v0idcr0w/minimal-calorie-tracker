<script>
    import { invoke } from "@tauri-apps/api";
    import { onMount } from "svelte";
    import { logId, foodsNormalized } from './store.js';
    import SingleRecipe from "./SingleRecipe.svelte";

    let newRecipe = {name: '', serving_size: 0.0, unit: ''}; 
    let newRecipeActive = false; 
    // Check if the new recipe box is properly filled 
    $: inputsFilled = newRecipe.name !== "" && newRecipe.serving_size >= 0.0 && newRecipe.unit !== "";
    // stores all recipes
    let recipes = []; 
    
    onMount(async () => {
        if ($foodsNormalized.length === 0) {
            foodsNormalized.set(await invoke('get_foods_normalized')); 
        }
        if (!$logId) {
            logId.set(await invoke('get_todays_log').id);
        }
        await getAllRecipes(); 
    });

    async function createNewRecipe() {
        await invoke('create_new_recipe', { name: newRecipe.name, servingSize: newRecipe.serving_size, unit: newRecipe.unit });
        // reset button status 
        newRecipeActive = false;
        // reset newRecipe object
        newRecipe = {name: '', serving_size: 0.0, unit: ''};   
        // refresh recipes 
        await getAllRecipes();
    }

    async function getAllRecipes() {
        recipes = await invoke('get_all_recipes');
        // sort by descending order of id 
        recipes.sort((a, b) => b.id - a.id); 
    }

    async function deleteRecipe(recipeId) {
        await invoke('delete_recipe', { recipeId });
        // refresh recipe list 
        await getAllRecipes(); 
    }


</script>

<!-- Creating a new recipe -->
<button on:click={() => newRecipeActive = !newRecipeActive}>Create New Recipe</button>
{#if newRecipeActive} 
<ul>
    <li>Name: <input type="text" name="recipeName" placeholder="Recipe Name" bind:value={newRecipe.name} /> </li>
    <li>Serving size and units: <input type="number" name="servingSize" placeholder=0 min=0 bind:value={newRecipe.serving_size} />  <input type="text" name="unit" placeholder="plate" bind:value={newRecipe.unit} /> </li>
</ul>

<button on:click={createNewRecipe} disabled={!inputsFilled}> Ok </button>
{/if}

<!-- Rendering all recipes below -->
{#each recipes as recipe (recipe.id)}
<div>
    <SingleRecipe recipe={recipe} onDelete={() => deleteRecipe(recipe.id)} />
</div>
{/each}