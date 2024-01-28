<script>
    import { invoke } from '@tauri-apps/api/tauri'
    import { foodsNormalized } from './store.js';

    let active = false; // track if the button is clicked (new food is being added by the user)

    // initialize new food to be filled in with data provided by the user 
    let newFood = { id: 0, name: '', serving_size: 0, unit: '', normalized_calories: 0, normalized_protein: 0, normalized_carbohydrate: 0, normalized_fat: 0 };

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
        if (newFood.normalized_protein < 0) {
            validationError.normalized_protein = 'Protein amount cannot be negative'
        }
        if (newFood.normalized_carbohydrate < 0) {
            validationError.normalized_carbohydrate = 'Carbohydrate amount cannot be negative'
        }
        if (newFood.normalized_fat < 0) {
            validationError.normalized_fat = 'Fat amount cannot be negative'
        }
        if (newFood.normalized_calories < 0) {
            validationError.normalized_calories = 'Calories cannot be negative'
        }
        return Object.keys(validationError).length === 0;
    }

    async function addNewFood() {

    // Make a validation check 
    if (validateNewFood(newFood)) {
        // Invoke backend code to add the new food to the database
        await invoke('add_new_food_normalized', { newFood });

        // Clear the input fields and reset the newFood object
        newFood = { id: 0, name: '', serving_size: 0, unit: '', normalized_calories: 0, normalized_protein: 0, normalized_carbohydrate: 0, normalized_fat: 0 };

        // Refresh the list of foods
        foodsNormalized.set(await invoke('get_foods_normalized'));
        }
    }

</script>

<div>
    <button on:click={() => active = !active }>
        {#if !active}
        Add New
        {:else}
        Cancel
        {/if}
    </button>

    {#if active}
        <div>
            <ul>
                <li><label for="name">Name:</label>
                    <input name="name" bind:value={newFood.name} />
                    {#if validationError.name}
                    <p class="error">{validationError.name}</p>
                    {/if}
                </li>
                <li>
                    <label for="serving_size">Serving Size:</label>
                    <input name="serving_size" type="number" min=0 bind:value={newFood.serving_size} />
                    {#if validationError.serving_size}
                    <p class="error">{validationError.serving_size}</p>
                    {/if}
                </li>
                <li>
                    <label for="unit">Unit:</label>
                    <input name="unit" bind:value={newFood.unit} />
                    {#if validationError.unit}
                    <p class="error">{validationError.unit}</p>
                    {/if}
                </li>
                <li>
                    <label for="calories">Calories:</label>
                    <input name="calories" type="number" min=0 bind:value={newFood.normalized_calories} />
                    {#if validationError.normalized_calories}
                    <p class="error">{validationError.normalized_calories}</p>
                    {/if}
                </li>
                <li>
                    <label for="protein">Protein:</label>
                     <input name="protein" type="number" min=0 bind:value={newFood.normalized_protein} />
                     {#if validationError.normalized_protein}
                    <p class="error">{validationError.normalized_protein}</p>
                    {/if}
                </li>
                <li>
                    <label for="carbohydrate">Carbohydrate:</label>
                    <input name="carbohydrate" type="number" min=0 bind:value={newFood.normalized_carbohydrate} />
                    {#if validationError.normalized_carbohydrate}
                    <p class="error">{validationError.normalized_carbohydrate}</p>
                    {/if}
                </li>
                <li>
                    <label for="fat">Fat:</label>
                    <input name="fat" type="number" min=0 bind:value={newFood.normalized_fat} />
                    {#if validationError.normalized_fat}
                    <p class="error">{validationError.normalized_fat}</p>
                    {/if}
                </li>
            </ul>
            <button on:click={addNewFood}>Add Food</button>
      </div>

    {/if}
</div>

<style>
    .error {
      color: red;
      margin-top: 0.2rem;
    }
  </style>