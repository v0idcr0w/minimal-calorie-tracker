<script>
    import { invoke } from "@tauri-apps/api";
    import { onMount } from "svelte";
    import { logId, today } from './store.js';

    let weight = 0; 
    let editWeight = false; 
    let todaysLog = {}; 

    async function getOrCreateTodaysLog() {
        todaysLog = await invoke('get_todays_log');  
        weight = todaysLog.weight; 
        logId.set(todaysLog.id); 
    }

    async function addWeight(weight) {
        if (validateInput(weight)) {
            await invoke('weight_in', { logId: $logId, weight });
            todaysLog.weight = weight;
            editWeight = false; 
        }  
    }

    onMount( async () => {
        await getOrCreateTodaysLog();
    });

    let validationError = ""; 
    function validateInput(weight) {
        if (weight < 0 || weight > 999) {
            validationError = "Weight must be between 0 and 999 kg"; 
            return false 
        }
        return true 
    }
</script>

<h3>Weight-in for {$today.toISOString().split('T')[0]}:</h3> 

{#if todaysLog.weight > 0}
    Your weight today is: {todaysLog.weight} kg
    <button on:click={() => editWeight = !editWeight}>{!editWeight ? "Edit" : "Cancel"}</button>
{/if} 
{#if todaysLog.weight <= 0 || editWeight}
    <p>Enter new weight:</p>
    <input type="number" name="weight" min=0 max=999 bind:value={weight} style="width: 60px;" /> kg
    <button on:click={addWeight(weight)}>Ok</button>
{/if}

{#if validationError}
    <p class="error">{validationError}</p>
{/if}


<h3>Your goals:</h3>