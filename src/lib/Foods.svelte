<script>
    // This displays all foods stored in their normalized format in the database. 
    import { invoke } from '@tauri-apps/api/tauri'
    import { onMount } from 'svelte'
    import { toTitleCase } from './titleCase.js'
    import { foodsNormalized } from './store.js';

    // components 
    import NewFood from './NewFood.svelte'

    let updatedFood = {}; 
    let editableArray = []; 

    async function refreshFoods() {
      foodsNormalized.set(await invoke('get_foods_normalized'))
      // initialize as array of false  
      editableArray = Array.from({ length: foodsNormalized.length }, () => false);
      updatedFood = {}
    }

    onMount(async () => {
      await refreshFoods() 
    });

    async function deleteFood(food) {
      await invoke('delete_food_normalized', { food })
      // refresh list 
      await refreshFoods()
    }

    async function updateFood(food) {
      await invoke('update_food_normalized', { food })
      await refreshFoods() 
    }
    
    function enableEditing(index, foodNormalized) {
      editableArray[index] = !editableArray[index]
      // copy object 
      updatedFood = {...foodNormalized}; 
    }

</script>
  
<div>
      {#each $foodsNormalized as foodNormalized, index (foodNormalized.id)}
    <div>
      <button on:click={() => enableEditing(index, foodNormalized) }>{ editableArray[index] ? "Revert Changes" : "Edit"}</button> 
      <button on:click={() => deleteFood(foodNormalized)}>Delete</button> 
      
        {#if !editableArray[index]}
        <h3>{toTitleCase(foodNormalized.name)} </h3>
        <li>Serving Size: {foodNormalized.serving_size} {foodNormalized.unit} </li>
        <li>Calories: {foodNormalized.normalized_calories} kcal</li>
        <li>Protein: {foodNormalized.normalized_protein} g</li>
        <li>Carbohydrates: {foodNormalized.normalized_carbohydrate} g </li>
        <li>Fats: {foodNormalized.normalized_fat} g</li>

        {:else}
        <h3 contenteditable="true" bind:textContent={updatedFood.name}>{toTitleCase(updatedFood.name)}</h3>
        <li>Serving Size:  
          <input name="serving_size" type="number" min=0 bind:value={updatedFood.serving_size} />
          <input name="unit" bind:value={updatedFood.unit} placeholder="Unit" />
        </li>
        <li>Calories: 
          <input name="calories_size" type="number" min=0 bind:value={updatedFood.normalized_calories} /> kcal
        </li>
        <li>Protein: 
          <input name="protein" type="number" min=0 bind:value={updatedFood.normalized_protein} /> g
        </li>
        <li>Carbohydrates: 
          <input name="carbohydrate" type="number" min=0 bind:value={updatedFood.normalized_carbohydrate} /> g
        </li>
        <li>Fats: 
          <input name="fat" type="number" min=0 bind:value={updatedFood.normalized_fat} /> g
        </li>

        <button on:click={() => updateFood(updatedFood)}>Confirm Changes</button> 

        {/if}
      </div>
      {/each}
    

      <NewFood />


</div>

<!-- TODO: make these buttons better. they look terrible and are completely out of position... -->