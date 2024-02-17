<script>
    import { invoke } from "@tauri-apps/api";
    import { onMount } from "svelte";
    import { logId, today, userGoal } from './store.js';
    import { toTitleCase } from './titleCase.js';
    import { base } from '$app/paths';
    import MacrosPie from './MacrosPie.svelte'; 
    
    let weight = 0; 
    // button controls 
    let editWeight = false;
    let editWeightGoal = false; 
    let editCaloriesGoal = false; 
    let editMacrosGoal = false; 
    
    let todaysLog = {}; 

    let newUserGoal = {}; 
    // this needs to be declared as reactive in order to make the chart reactive too 
    $: totalCalories = newUserGoal.protein * 4 + newUserGoal.carbohydrate * 4 + newUserGoal.fat * 9;  
    $: totalMacros = newUserGoal.protein + newUserGoal.carbohydrate + newUserGoal.fat; 
    $: macros = [newUserGoal.protein/totalMacros * 100, newUserGoal.carbohydrate/totalMacros * 100, newUserGoal.fat/totalMacros * 100 ]; 
    $: validMacros = totalCalories <= newUserGoal.calories;

    async function getOrCreateTodaysLog() {
        todaysLog = await invoke('get_todays_log');  
        weight = todaysLog.weight; 
        logId.set(todaysLog.id); 
    }

    async function getOrCreateUserGoal() {
        // if the user is launching the app for the first time, this will create the user goal
        userGoal.set(await invoke('get_user_goal')); 
        newUserGoal = {...$userGoal}; // copy the object 
    }

    async function addWeight(weight) {
        if (validateInput(weight)) {
            await invoke('weight_in', { logId: $logId, weight });
            todaysLog.weight = weight;
            editWeight = false; 
        }  
    }

    // functions to edit the user's goal 
    async function updateWeightGoal() {
        userGoal.set(await invoke('update_weight_goal', { newUserGoal }));
        editWeightGoal = false;  
    }

    async function updateCaloriesGoal() {
        userGoal.set(await invoke('update_calories_goal', { newUserGoal }));
        editCaloriesGoal = false; 
    }

    async function updateMacrosGoal() {
        userGoal.set(await invoke('update_macros_goal', { newUserGoal }));
        editMacrosGoal = false; 
    }

    onMount( async () => {
        await getOrCreateTodaysLog();
        await getOrCreateUserGoal(); 
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

<h3>Weight-in for {$today.toISOString().split('T')[0]}:
    {#if todaysLog.weight > 0}
        {todaysLog.weight} kg
        <button class="icon-button" on:click={() => editWeight = !editWeight}>
            {#if !editWeight}
            <img src="{`${base}/edit.svg`}" alt="Edit" width="30" />
            <span class="tooltiptext">Edit today's weight</span>
            {:else}
            <img src="{`${base}/cancel.svg`}" alt="Cancel" width="30" />
            {/if}
        </button>
    {/if} 
</h3> 


<div>
{#if todaysLog.weight <= 0 || editWeight}
    <p>Enter new weight:
    <input type="number" name="weight" min=0 max=999 bind:value={weight} style="width: 60px;" /> kg
    <button class="icon-button" on:click={addWeight(weight)}>
        <img src="{`${base}/ok.svg`}" alt="Confirm" width="30" />
    </button>
</p>
{/if}

{#if validationError}
    <p class="error">{validationError}</p>
{/if}
</div>


<div>
<h3>Your goals</h3>

<h4>Weight 
    <button class="icon-button" on:click={() => editWeightGoal = !editWeightGoal }>
    {#if !editWeightGoal} 
    <img src="{`${base}/edit.svg`}" alt="Edit" width="30" />
    <span class="tooltiptext">Edit weight goal</span>
    {:else}
    <img src="{`${base}/cancel.svg`}" alt="Cancel" width="30" />
    {/if}
</button> </h4> 


{#if editWeightGoal}
    <input type="radio" id="w1" name="choice" value="lose" bind:group={newUserGoal.weight}> <label for="w1">Lose Weight</label>
        {#if newUserGoal.weight == 'lose'}
            <input class="small-input" type="number" min=0 bind:value={newUserGoal.weight_rate} >% per week
        {/if}
    <br/>

    <input type="radio" id="w2" name="choice" value="gain" bind:group={newUserGoal.weight}> <label for="w2">Gain Weight</label>
        {#if newUserGoal.weight == 'gain'}
            <input class="small-input" type="number" min=0 bind:value={newUserGoal.weight_rate} >% per week
        {/if} 
    <br/>

    <input type="radio" id="w3" name="choice" value="maintain" bind:group={newUserGoal.weight}>
    <label for="w3">Maintain Weight</label><br/>
    <button class="icon-button" on:click={updateWeightGoal}>
        <img src="{`${base}/ok.svg`}" alt="Confirm" width="30" />
    </button>
{:else}
    <p>{$userGoal.weight ? toTitleCase($userGoal.weight) : ""} weight at a rate of {$userGoal.weight_rate ? $userGoal.weight_rate.toFixed(2) : 0}% per week</p>
{/if}
</div>


<div>
    <h4>Daily Calories 
        <button class="icon-button" on:click={() => editCaloriesGoal = !editCaloriesGoal}>
            {#if !editCaloriesGoal}
            <img src="{`${base}/edit.svg`}" alt="Edit" width="30" />
            <span class="tooltiptext">Edit calories goal</span>
            {:else}
            <img src="{`${base}/cancel.svg`}" alt="Cancel" width="30" />
            {/if}
        </button>
    </h4>

    <p>
    {#if editCaloriesGoal}
        <input type="number" min=0 bind:value={newUserGoal.calories}> kcal
        <button class="icon-button" on:click={updateCaloriesGoal}>
            <img src="{`${base}/ok.svg`}" alt="Confirm" width="30" />
        </button>
    {:else}
        {$userGoal.calories} kcal
    {/if}
    </p>
</div>

<div class="macros-container">
    <div style="margin-right: 60px;">
    <h4>Macronutrients 
        <button class="icon-button" on:click={() => editMacrosGoal = !editMacrosGoal}>
            {#if !editMacrosGoal}
            <img src="{`${base}/edit.svg`}" alt="Edit" width="30" />
            <span class="tooltiptext">Edit macronutrients goal</span>
            {:else}
            <img src="{`${base}/cancel.svg`}" alt="Cancel" width="30" />
            {/if}
        </button>
    </h4>
    {#if editMacrosGoal}
        <input class="small-input" type="number" min=0 bind:value={newUserGoal.protein}> g Protein ≈ {newUserGoal.protein * 4} kcal <br/>
        <input class="small-input" type="number" min=0 bind:value={newUserGoal.carbohydrate}> g Carbohydrates ≈ {newUserGoal.carbohydrate * 4} kcal <br/>
        <input class="small-input" type="number" min=0 bind:value={newUserGoal.fat}> g Fats ≈ {newUserGoal.fat * 9} kcal <br/>
        {#if !validMacros}
            <p class="error">Total macronutrients exceeds daily calorie goal by {totalCalories - newUserGoal.calories} kcal. You may want to update your goals.</p>
        {/if}
        <button class="icon-button" on:click={updateMacrosGoal}>
            <img src="{`${base}/ok.svg`}" alt="Confirm" width="30" />
        </button>
    {:else}
        <p>{$userGoal.protein} g Protein</p>
        <p>{$userGoal.carbohydrate} g Carbohydrates</p>
        <p>{$userGoal.fat} g Fats</p> 
    {/if}
    </div>
    <MacrosPie {macros}/> 
</div>
