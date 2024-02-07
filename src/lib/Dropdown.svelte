<script>
    export let onAdd; 
    export let options; 

    let selectedFood = {}; 
    let amount;

    function handleSelect(event) {
        const selectedId = event.target.value; // value is a string 
        const food = options.find((item) => item.id == selectedId); 
        selectedFood = food; 
    }


</script>
<label for="foodDropdown">Select a food:</label>
<select id="foodDropdown" on:change={handleSelect}>
<!-- first option is blank and unselectable -->
<option value="" disabled selected hidden></option>
{#each options as option (option.id)}
    <option value={option.id}>{option.name}</option>
{/each}
</select>

{#if Object.keys(selectedFood).length !== 0}
Select amount: <input type="number" name="amount" placeholder="{selectedFood.serving_size}" min=0 bind:value={amount} /> {selectedFood.unit}
<button on:click={onAdd(selectedFood.id, amount)} disabled={!(amount >= 0)} >Add</button>

{/if}
