<script>
    import { invoke } from "@tauri-apps/api";
    import { onMount } from "svelte";
    import { foodsNormalized } from "./store";

    // receive the mealId prop
    export let mealId; 
    // check if foodsNormalized is empty 
    onMount(async () => {
        // this element is responsible only for rendering the dropdown for meals. no need to query the database again. 
        if ($foodsNormalized.length === 0) {
            foodsNormalized.set(await invoke('get_foods_normalized')); 
        }
    });

    let selectedFood = {}; 
    let amount;
    let validationError = ""; 

    function handleSelect(event) {
        const selectedId = event.target.value; // value is a string 
        const food = $foodsNormalized.find((item) => item.id == selectedId); 
        selectedFood = food; 
    }

    async function addNewFood(selectedId, amount) {
        if (amount > 0) {
            await invoke('add_new_food', { selectedId, amount, mealId })
            // dispatch this event when a new food gets added 
            const event = new CustomEvent('foodAdded');
            dispatchEvent(event); 
        } else {
            validationError = "Amount must be greater than 0"
        }
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
<button on:click={addNewFood(selectedFood.id, amount)}>Add</button>

{#if validationError}
<p class="error">{validationError}</p>
{/if}
{/if}
