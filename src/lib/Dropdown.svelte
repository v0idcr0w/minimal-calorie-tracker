<script>
    import { invoke } from "@tauri-apps/api";
    import { onMount } from "svelte";
    import { foodsNormalized } from "./store";

    // props
    // export let mealId;  
    export let onAdd; 

    // check if foodsNormalized is empty 
    onMount(async () => {
        // this element is responsible only for rendering the dropdown for meals. no need to query the database again. 
        if ($foodsNormalized.length === 0) {
            foodsNormalized.set(await invoke('get_foods_normalized')); 
        }
    });

    let selectedFood = {}; 
    let amount;

    function handleSelect(event) {
        const selectedId = event.target.value; // value is a string 
        const food = $foodsNormalized.find((item) => item.id == selectedId); 
        selectedFood = food; 
    }


</script>
<label for="foodDropdown">Select a food:</label>
<select id="foodDropdown" on:change={handleSelect}>
<!-- first option is blank and unselectable -->
<option value="" disabled selected hidden></option>
{#each $foodsNormalized as foodNormalized (foodNormalized.id)}
    <option value={foodNormalized.id}>{foodNormalized.name}</option>
{/each}
</select>

{#if Object.keys(selectedFood).length !== 0}
Select amount: <input type="number" name="amount" placeholder="{selectedFood.serving_size}" min=0 bind:value={amount} /> {selectedFood.unit}
<button on:click={onAdd(selectedFood.id, amount)} disabled={!(amount >= 0)} >Add</button>

{/if}
