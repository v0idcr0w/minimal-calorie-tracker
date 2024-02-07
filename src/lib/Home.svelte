<script>
    import { invoke } from "@tauri-apps/api";
    import { onMount } from "svelte";
    import { logId, today } from './store.js';
    import { toTitleCase } from './titleCase.js';

    let weight = 0; 
    // button controls 
    let editWeight = false;
    let editWeightGoal = false; 
    let editCaloriesGoal = false; 
    let editMacrosGoal = false; 
    
    let todaysLog = {}; 
    // to be replaced 
    let userGoals = {weight: 'gain', rate: 0, calories: 1500, protein: 150, carbohydrate: 200, fat: 50}; 

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
<h4>Weight <button on:click={() => editWeightGoal = !editWeightGoal }>{editWeightGoal ? "Cancel" : "Edit"}</button> </h4> 

{#if editWeightGoal}
    <input type="radio" id="w1" name="choice" value="lose" bind:group={userGoals.weight}> <label for="w1">Lose Weight</label>
        {#if userGoals.weight == 'lose'}
            <input type="number" min=0 bind:value={userGoals.rate} >% per week
        {/if}
    <br/>

    <input type="radio" id="w2" name="choice" value="gain" bind:group={userGoals.weight}> <label for="w2">Gain Weight</label>
        {#if userGoals.weight == 'gain'}
            <input type="number" min=0 bind:value={userGoals.rate} >% per week
        {/if} 
    <br/>

    <input type="radio" id="w3" name="choice" value="maintain" bind:group={userGoals.weight}>
    <label for="w3">Maintain Weight</label><br/>
{:else}
    <p>{toTitleCase(userGoals.weight)} weight at a rate of {userGoals.rate.toFixed(2)}% per week</p>

{/if}

<h4>Daily Calories <button on:click={() => editCaloriesGoal = !editCaloriesGoal}>{editCaloriesGoal ? "Cancel" : "Update"}</button></h4> 
{#if editCaloriesGoal}
    <input type="number" min=0 bind:value={userGoals.calories}> kcal
{:else}
    <p>{userGoals.calories} kcal</p>
{/if}


<h4>Macronutrients  <button on:click={() => editMacrosGoal = !editMacrosGoal}>{editMacrosGoal ? "Cancel" : "Update"}</button> </h4>
{#if editMacrosGoal}
    <input type="number" min=0 bind:value={userGoals.protein}> g Protein <br/>
    <input type="number" min=0 bind:value={userGoals.carbohydrate}> g Carbohydrates <br/>
    <input type="number" min=0 bind:value={userGoals.fat}> g Fats <br/>
{:else}
    <p>{userGoals.protein} g Protein</p>
    <p>{userGoals.carbohydrate} g Carbohydrates</p>
    <p>{userGoals.fat} g Fats</p> 
{/if}
