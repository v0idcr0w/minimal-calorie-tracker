<script>
    // this part contains the logic related to rendering foods, editing and deleting a food from a given meal. 
    import { invoke } from "@tauri-apps/api";
    import { onMount, onDestroy } from "svelte";
    import { toTitleCase } from "./titleCase";
    import Dropdown from "./Dropdown.svelte";

    // props 
    export let mealId; 
    export let onUpdate; 

    // arrays to store food information 
    let foods = []; 
    let editableArray = []; 
    let newAmountArray = [];  
    let computedMacros = {}; 

    // button control
    let dropdownActive = false; 

    async function refreshFoods() {
        foods = await invoke('get_foods_by_meal_id', { mealId }); 
        computedMacros = await invoke('compute_meal_macros', { mealId }); 
        editableArray = new Array(foods.length).fill(false);
        newAmountArray = foods.map((food) => food.amount);
    }

    onMount(async () => {
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
        dropdownActive = false; 
    }
</script>


<table>
{#each foods as food, index (food.id)} 
    <tr>
        <td>
            <button on:click={deleteFood(food)}>-</button>
        </td>
        <td>{toTitleCase(food.name)}</td>
        
        {#if !editableArray[index]}
            <td>{food.amount}</td>
        {:else}
            <td><input type="number" min=0 bind:value={newAmountArray[index]} placeholder={food.amount} style="width: 3em" /></td>
        {/if}
        <td>{food.unit}</td>
        <td>
            <button on:click={() => editableArray[index] = !editableArray[index] }>{ editableArray[index] ? "Cancel" : "Edit"}</button>
        </td>
        <td>{food.calories.toFixed(0)}</td>
        <td>kcal</td>
        {#if editableArray[index]}
        <td>
            <button on:click={updateFood(food, newAmountArray[index])} disabled={!(newAmountArray[index] >= 0)} >Ok</button>
        </td>
        {/if}
    </tr>

{/each}
</table>


<!-- Dropdown button for foods list -->
<button on:click={() => dropdownActive = !dropdownActive}>{dropdownActive ? "Cancel" : "+"}</button>
{#if dropdownActive}
    <Dropdown onAdd={addNewFood} />
{/if}


<!-- Table containing the total amount of macros for the meal -->
{#if Object.keys(computedMacros).length !== 0}
<table>
    <thead>
        <tr>
            <th colspan="3">Total</th>
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