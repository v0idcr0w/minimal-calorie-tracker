<script>
    import { base } from '$app/paths'; 
	import MacrosPie from './MacrosPie.svelte';
	import MaterialFloatingLabel from './MaterialFloatingLabel.svelte';
	import { toTitleCase } from './titleCase';

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

<div>
<select class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 w-1/2 p-1" id="foodDropdown" on:change={handleSelect}>
<!-- first option is blank and unselectable -->
<option value="" disabled selected hidden></option>
{#each options as option (option.id)}
    <option value={option.id}>{toTitleCase(option.name)}</option>
{/each}
</select>

{#if Object.keys(selectedFood).length !== 0}
<div class="flex flex-col items-center  text-left  mt-2"> 
    <MaterialFloatingLabel label="Amount ({selectedFood.unit})" bind:value={amount} />
</div>

    <div class="flex justify-center">
    <button class="text-button w-32 p-1 mx-2" on:click={onAdd(selectedFood.id, Number(amount))} disabled={!(Number(amount) >= 0)}>
        <svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 -960 960 960" width="24" fill="currentColor"><path d="M382-240 154-468l57-57 171 171 367-367 57 57-424 424Z"/></svg> OK
    </button>
    </div>
    {/if}
</div>
