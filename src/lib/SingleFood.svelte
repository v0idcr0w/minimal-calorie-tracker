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


<h3>{toTitleCase(foodNormalized.name)}
    <button on:click={() => editableName = !editableName}>Rename</button> 
    {#if editableName}
    <input name="name" type="text" bind:value={updatedFood.name} placeholder={foodNormalized.name} />
    <button on:click={updateFoodName(foodNormalized, updatedFood.name)}>Ok</button>
    {/if}
    </h3> 
    <!-- Button that controls editing the rest of the fields -->
    <button on:click={() => enableEditing(foodNormalized) }>{ editableFields ? "Revert Changes" : "Edit"}</button> 
    <button on:click={() => onDelete(foodNormalized)}>Delete</button> 
    
    {#if !editableFields}
    <table>
      <tr>
        <td>Serving Size</td> 
        <td>{foodNormalized.serving_size}</td>
        <td>{foodNormalized.unit}</td>
      </tr>
      <tr>
        <td>Calories</td>
        <td>{foodNormalized.normalized_calories}</td>
        <td>kcal</td>
      </tr>
      <tr>
        <td>Protein</td>
        <td>{foodNormalized.normalized_protein}</td>
        <td>g</td>
      </tr>
      <tr>
        <td>Carbohydrates</td>
        <td>{foodNormalized.normalized_carbohydrate}</td>
        <td>g</td>
      </tr>
      <tr>
        <td>Fats</td>
        <td>{foodNormalized.normalized_fat}</td>
        <td>g</td>
      </tr>
    </table>

    {:else}
    <table>
      <tr>
        <td>Serving Size</td> 
        <td><input class="small-input" name="serving_size" type="number" min=0 bind:value={updatedFood.serving_size} /></td>
        <td><input class="small-input" name="unit" bind:value={updatedFood.unit} placeholder="Unit" /></td>
      </tr>
      <tr>
        <td>Calories</td>
        <td><input class="small-input" name="calories_size" type="number" min=0 bind:value={updatedFood.normalized_calories} /></td>
        <td>kcal</td>
      </tr>
      <tr>
        <td>Protein</td>
        <td> <input class="small-input" name="protein" type="number" min=0 bind:value={updatedFood.normalized_protein} /> </td>
        <td>g</td>
      </tr>
      <tr>
        <td>Carbohydrates</td>
        <td><input class="small-input" name="carbohydrate" type="number" min=0 bind:value={updatedFood.normalized_carbohydrate} /></td>
        <td>g</td>
      </tr>
      <tr>
        <td>Fats</td>
        <td><input class="small-input" name="fat" type="number" min=0 bind:value={updatedFood.normalized_fat} /></td>
        <td>g</td>
      </tr>
    </table>
      <button on:click={() => updateFood(updatedFood)}>Confirm Changes</button> 

    {/if}


<style>
    .small-input {
      width: 50px;
    }
  </style>