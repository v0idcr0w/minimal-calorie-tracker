<script>
    import { invoke } from '@tauri-apps/api/tauri'; 
    import { toTitleCase } from './titleCase.js'; 

    // props 
    export let foodNormalized; 
    export let onDelete; 

    let updatedFood = {}; 
    let editableName = false; 
    let editableFields = false; 

    function enableEditing(foodNormalized) {
      editableFields = !editableFields; 
      updatedFood = {...foodNormalized}; 
    }

    async function updateFood(food) {
      foodNormalized = await invoke('update_food_normalized', { food }); 
      editableFields = false; 
    }

    async function updateFoodName(food, newName) {
        await invoke('update_food_normalized_name', { food, newName }); 
        editableName = false; 
        foodNormalized.name = newName; 
    } 
</script>

<div class="block w-full text-center tracking-tighter rounded-lg bg-white p-2 shadow-[0_2px_15px_-3px_rgba(0,0,0,0.07),0_10px_20px_-2px_rgba(0,0,0,0.04)]">

<h3 class="text-neutral-700 text-xl m-4 font-bold" on:dblclick={() => editableName = true} >
  {#if !editableName}
  {toTitleCase(foodNormalized.name)}
  {/if}

    {#if editableName}
    <input name="name" type="text" bind:value={updatedFood.name} placeholder={foodNormalized.name} class="w-1/2" />
    <!-- Accept changes button -->
    <button class="relative inline-flex items-center justify-center p-0.5 mb-2 me-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-purple-600 to-blue-500 group-hover:from-purple-600 group-hover:to-blue-500 hover:text-white focus:ring-4 focus:outline-none focus:ring-blue-300" on:click={updateFoodName(foodNormalized, updatedFood.name)}>
      <span class="relative transition-all ease-in duration-75 bg-white rounded-md group-hover:bg-opacity-0">
        <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="M382-240 154-468l57-57 171 171 367-367 57 57-424 424Z"/></svg>
        </span>
    </button>

    <!-- Cancel changes button -->
    <button class="relative inline-flex items-center justify-center p-0.5 mb-2 me-2 overflow-hidden text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-purple-600 to-blue-500 group-hover:from-purple-600 group-hover:to-blue-500 hover:text-white focus:ring-4 focus:outline-none focus:ring-blue-300" on:click={() => editableName = !editableName}>
      <span class="relative transition-all ease-in duration-75 bg-white rounded-md group-hover:bg-opacity-0">
        <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor" ><path d="m256-200-56-56 224-224-224-224 56-56 224 224 224-224 56 56-224 224 224 224-56 56-224-224-224 224Z"/></svg>
      </span>
    </button> 
    {/if}
</h3> 
    <!-- Button that controls editing the rest of the fields -->
    <p>
      <button class="icon-button" on:click={() => enableEditing(foodNormalized) }>
        {#if !editableFields}
        <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="M200-200h57l391-391-57-57-391 391v57Zm-80 80v-170l528-527q12-11 26.5-17t30.5-6q16 0 31 6t26 18l55 56q12 11 17.5 26t5.5 30q0 16-5.5 30.5T817-647L290-120H120Zm640-584-56-56 56 56Zm-141 85-28-29 57 57-29-28Z"/></svg>
        {:else}
        <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor" ><path d="m256-200-56-56 224-224-224-224 56-56 224 224 224-224 56 56-224 224 224 224-56 56-224-224-224 224Z"/></svg>
        {/if}
      </button> 
      <button class="icon-button mb-2 mx-2" on:click={() => onDelete(foodNormalized)}>
        <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="M280-120q-33 0-56.5-23.5T200-200v-520h-40v-80h200v-40h240v40h200v80h-40v520q0 33-23.5 56.5T680-120H280Zm400-600H280v520h400v-520ZM360-280h80v-360h-80v360Zm160 0h80v-360h-80v360ZM280-720v520-520Z"/></svg>
      </button> 
    </p>


    <!-- TABLES BELOW -->

    {#if !editableFields}
    <table class="mx-auto">
      <tr>
        <td class="whitespace-nowrap px-2 py-0.5 font-bold">Serving Size</td> 
        <td>{foodNormalized.serving_size}</td>
        <td class="text-right">{foodNormalized.unit}</td>
      </tr>
      <tr>
        <td class="whitespace-nowrap px-2 py-0.5 font-bold">Calories</td>
        <td>{foodNormalized.normalized_calories}</td>
        <td class="text-right">kcal</td>
      </tr>
      <tr>
        <td class="whitespace-nowrap px-2 py-0.5 font-bold">Protein</td>
        <td>{foodNormalized.normalized_protein}</td>
        <td class="text-right">g</td>
      </tr>
      <tr>
        <td class="whitespace-nowrap px-2 py-0.5 font-bold">Carbohydrates</td>
        <td>{foodNormalized.normalized_carbohydrate}</td>
        <td class="text-right">g</td>
      </tr>
      <tr>
        <td class="whitespace-nowrap px-2 py-0.5 font-bold">Fats</td>
        <td>{foodNormalized.normalized_fat}</td>
        <td class="text-right">g</td>
      </tr>
    </table>

    {:else}
    <table class="mx-auto">
      <tr>
        <td class="whitespace-nowrap px-2 py-0.5 font-bold">Serving Size</td> 
        <td><input class="w-full" name="serving_size" type="number" min=0 bind:value={updatedFood.serving_size} /></td>
        <td class="text-right"><input class="w-full" name="unit" bind:value={updatedFood.unit} placeholder="Unit" /></td>
      </tr>
      <tr>
        <td class="whitespace-nowrap px-2 py-0.5 font-bold">Calories</td>
        <td><input class="w-full" name="calories_size" type="number" min=0 bind:value={updatedFood.normalized_calories} /></td>
        <td>kcal</td>
      </tr>
      <tr>
        <td class="whitespace-nowrap px-2 py-0.5 font-bold">Protein</td>
        <td> <input class="w-full" name="protein" type="number" min=0 bind:value={updatedFood.normalized_protein} /> </td>
        <td>g</td>
      </tr>
      <tr>
        <td class="whitespace-nowrap px-2 py-0.5 font-bold">Carbohydrates</td>
        <td><input class="w-full" name="carbohydrate" type="number" min=0 bind:value={updatedFood.normalized_carbohydrate} /></td>
        <td>g</td>
      </tr>
      <tr>
        <td class="whitespace-nowrap px-2 py-0.5 font-bold">Fats</td>
        <td><input class="w-full" name="fat" type="number" min=0 bind:value={updatedFood.normalized_fat} /></td>
        <td>g</td>
      </tr>
    </table>
      <button class="icon-button" on:click={() => updateFood(updatedFood)}>
        <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="M382-240 154-468l57-57 171 171 367-367 57 57-424 424Z"/></svg>
    </button> 

    {/if}
</div>
