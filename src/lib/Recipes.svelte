<script>
    import { invoke } from "@tauri-apps/api";
    import { confirm } from '@tauri-apps/api/dialog';
    import { onMount } from "svelte";
    import { logId, foodsNormalized, recipes } from './store.js';
    import SingleRecipe from "./SingleRecipe.svelte";
	import MaterialFloatingLabel from "./MaterialFloatingLabel.svelte";

    let newRecipe = {name: '', serving_size: 0.0, unit: ''}; 
    let newRecipeActive = false; 
    // Check if the new recipe box is properly filled 
    $: inputsFilled = newRecipe.name !== "" && Number(newRecipe.serving_size) >= 0.0 && newRecipe.unit !== "";
    console.log(newRecipe.serving_size)
    
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
        await invoke('create_new_recipe', { name: newRecipe.name, servingSize: Number(newRecipe.serving_size), unit: newRecipe.unit });
        // reset button status 
        newRecipeActive = false;
        // reset newRecipe object
        newRecipe = {name: '', serving_size: 0.0, unit: ''};   
        // refresh recipes 
        await getAllRecipes();
    }

    async function getAllRecipes() {
        recipes.set(await invoke('get_all_recipes'));
        // sort by descending order of id 
        $recipes.sort((a, b) => b.id - a.id); 
    }

    async function deleteRecipe(recipeId) {
        const confirmed = await confirm('This action cannot be reverted. Are you sure?', { title: 'Confirm', type: 'info' });
        if (!confirmed) return; 
        await invoke('delete_recipe', { recipeId });
        // refresh recipe list 
        await getAllRecipes(); 
    }


</script>

<div class="mx-4">
    <!-- Creating a new recipe -->
    <div class="flex flex-col items-center justify-center">
    <div class="mb-4">
        <div class="flex justify-center">
        <button class="text-button mb-4" on:click={() => newRecipeActive = !newRecipeActive}>
            {#if !newRecipeActive}
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="M440-440H200v-80h240v-240h80v240h240v80H520v240h-80v-240Z"/></svg> New recipe 
            {:else}
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="m256-200-56-56 224-224-224-224 56-56 224 224 224-224 56 56-224 224 224 224-56 56-224-224-224 224Z"/></svg> Cancel
            {/if}
        </button>
        </div>
    
        {#if newRecipeActive} 
        <div class="block tracking-tighter text-sm">
            <table class="mx-auto">
                <tr>
                    <td colspan="2"> <MaterialFloatingLabel label="Recipe name" bind:value={newRecipe.name} /> </td>
                </tr>
                <tr>
                    <td><MaterialFloatingLabel label="Serving size" bind:value="{newRecipe.serving_size}" /></td>
                    <td>
                        <MaterialFloatingLabel label="Measurement unit" bind:value="{newRecipe.unit}" />
                    </td>
                </tr>
            </table>

            <!-- Flex justify-center is necessary to center these buttons  -->
            <div class="flex justify-center">
            <button class="text-button" on:click={createNewRecipe} disabled={!inputsFilled} > 
                <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="M382-240 154-468l57-57 171 171 367-367 57 57-424 424Z"/></svg> OK
            </button>
            </div>
        </div>
        {/if}
        </div>
    </div>


<!-- Rendering all recipes below -->
    <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
    {#each $recipes as recipe (recipe.id)}
        <SingleRecipe recipe={recipe} onDelete={() => deleteRecipe(recipe.id)} />
    {/each}
    </div>

</div>