<script>
    import { invoke } from '@tauri-apps/api/tauri'
    import { foodsNormalized } from './store.js';
    import MaterialFloatingLabel from './MaterialFloatingLabel.svelte';
    import MaterialFloatingLabelError from './MaterialFloatingLabelError.svelte';

    let active = false; // track if the button is clicked (new food is being added by the user)

    // initialize new food to be filled in with data provided by the user 
    let newFood = { id: 0, name: '', serving_size: '', unit: '', normalized_calories: '', normalized_protein: '', normalized_carbohydrate: '', normalized_fat: '' };

    // validation error object 
    let validationError = {}; 

    function validateNewFood(newFood) {
        if (!newFood.name.trim()) {
            validationError.name = 'Food name is required'; 
        }
        if (newFood.serving_size <= 0) {
            validationError.serving_size = 'Serving size must be greater than 0'
        }
        if (!newFood.unit) {
            validationError.unit = 'Unit of measurement must be provided'
        }
        if (!newFood.normalized_protein || Number(newFood.normalized_protein) < 0) {
            validationError.normalized_protein = 'Protein amount cannot be negative'
        }
        if (!newFood.normalized_protein || Number(newFood.normalized_carbohydrate) < 0) {
            validationError.normalized_carbohydrate = 'Carbohydrate amount cannot be negative'
        }
        if (!newFood.normalized_fat || Number(newFood.normalized_fat) < 0) {
            validationError.normalized_fat = 'Fat amount cannot be negative'
        }
        if (!newFood.normalized_calories || Number(newFood.normalized_calories) < 0) {
            validationError.normalized_calories = 'Calories cannot be negative'
        }
        return Object.keys(validationError).length === 0;
    }

    async function addNewFood() {

    // Make a validation check 
    if (validateNewFood(newFood)) {
        // Invoke backend code to add the new food to the database

        newFood = { ...newFood, serving_size: Number(newFood.serving_size), normalized_calories: Number(newFood.normalized_calories), normalized_protein: Number(newFood.normalized_protein), normalized_carbohydrate: Number(newFood.normalized_carbohydrate), normalized_fat: Number(newFood.normalized_fat)}; 

        await invoke('add_new_food_normalized', { newFood });

        // Clear the input fields and reset the newFood object
        newFood = { id: 0, name: '', serving_size: 0, unit: '', normalized_calories: 0, normalized_protein: 0, normalized_carbohydrate: 0, normalized_fat: 0 };

        // Refresh the list of foods
        foodsNormalized.set(await invoke('get_foods_normalized'));
        }
        active = false; 
    }

</script>

<div class="flex flex-col items-center">
    <div class="mt-4 mb-4">
        <button on:click={() => active = !active } class="text-button">
            {#if !active}
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="M440-440H200v-80h240v-240h80v240h240v80H520v240h-80v-240Z"/></svg> New food 
            {:else}
            <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="m256-200-56-56 224-224-224-224 56-56 224 224 224-224 56 56-224 224 224 224-56 56-224-224-224 224Z"/></svg> Cancel
            {/if}
        </button>
    </div>

    {#if active}
        <div class="block tracking-tighter text-sm">
            <table class="mx-auto">
                <tr>
                    {#if !validationError.name}
                    <td colspan="2"><MaterialFloatingLabel label="Food name"  bind:value={newFood.name} /></td>
                    {:else}
                    <td colspan="2"><MaterialFloatingLabelError error="No name" errorMessage={validationError.name} bind:value={newFood.name} /></td>
                    {/if}
                </tr>

                <tr>
                    {#if !validationError.serving_size}
                    <td> <MaterialFloatingLabel label="Serving size" bind:value={newFood.serving_size} /> </td>
                    {:else}
                    <td> <MaterialFloatingLabelError  error="No serving size" errorMessage={validationError.serving_size} bind:value={newFood.serving_size} /> </td>
                    {/if}
                    {#if !validationError.unit}
                    <td> <MaterialFloatingLabel label="Measurement unit" bind:value={newFood.unit}  /> </td>
                    {:else} 
                    <td> <MaterialFloatingLabelError error="No unit" errorMessage={validationError.unit} bind:value={newFood.unit} /> </td>
                    {/if}
                </tr>

                <tr>
                    {#if !validationError.normalized_calories}
                    <td> <MaterialFloatingLabel label="Calories (kcal)" bind:value={newFood.normalized_calories} />  </td>
                    {:else}
                    <td> <MaterialFloatingLabelError bind:value={newFood.normalized_calories} error="Invalid amount" errorMessage={validationError.normalized_calories} /> </td>
                    {/if}
                    
                    {#if !validationError.normalized_protein}
                    <td> <MaterialFloatingLabel label="Protein amount (g)" bind:value={newFood.normalized_protein}/> </td>
                    {:else}
                    <td> <MaterialFloatingLabelError bind:value={newFood.normalized_protein} error="Invalid amount" errorMessage={validationError.normalized_protein} /> </td>
                    {/if}
                </tr>

                <tr>
                    {#if !validationError.normalized_carbohydrate}
                    <td> <MaterialFloatingLabel label="Carbohydrate amount (g)" bind:value={newFood.normalized_carbohydrate} /> </td> 
                    {:else}
                    <td> <MaterialFloatingLabelError bind:value={newFood.normalized_carbohydrate} error="Invalid amount" errorMessage={validationError.normalized_carbohydrate} /> </td>
                    {/if}
                    
                    {#if !validationError.normalized_fat}
                    <td> <MaterialFloatingLabel label="Fat amount (g)" bind:value={newFood.normalized_fat} /> </td>
                    {:else}
                    <td> <MaterialFloatingLabelError bind:value={newFood.normalized_fat} error="Invalid amount" errorMessage={validationError.normalized_fat} /> </td>
                    {/if}
                </tr>
            </table>
        </div>
            <div>
                <button on:click={addNewFood} class="text-button">
                    <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="M382-240 154-468l57-57 171 171 367-367 57 57-424 424Z"/></svg> OK
                </button>
            </div>
      

    {/if}
</div>
