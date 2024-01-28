<script>
    import { invoke } from '@tauri-apps/api/tauri'
    import { onMount } from 'svelte'
    import { toTitleCase } from './titleCase.js';
    import Dropdown from './Dropdown.svelte';
    import SingleMeal from './SingleMeal.svelte'; 

    let date = "01/01/2024";
    let meals = []; 
    let newMeal = {}; 
    let createMealActive = false; 
    let dropdownsActive = []; 

    async function refreshMeals() {
        meals = await invoke('get_meals');
        dropdownsActive = Array.from({ length: meals.length }, () => false);
        newMeal = {id: 0, meal_type: ''};
    }

    async function addNewMeal(newMeal) {
        await invoke('add_new_meal', { newMeal })
        await refreshMeals(); 
    }

    onMount(async () => {
      await refreshMeals(); 
    });


</script>

<h3>Calorie and macronutrient consumption on {date}</h3>

<button on:click={() => createMealActive = !createMealActive}>{ createMealActive ? "Cancel" : "Create Meal" }</button>

{#if createMealActive}
<input name="type" placeholder="Meal type" bind:value={newMeal.meal_type} />
<button on:click={addNewMeal(newMeal)}>Confirm</button>
{/if}

{#each meals as meal, index (meal.id) } 
<p><b>{toTitleCase(meal.meal_type)}</b></p> 
<!-- clicking this button expands the dropdown list -->
<button on:click={() => dropdownsActive[index] = !dropdownsActive[index]}> {dropdownsActive[index] ? "-" : "+"} </button>
{#if dropdownsActive[index]}
<Dropdown mealId={meal.id}/>
{/if}
<!-- render all foods associated with this meal -->
<SingleMeal mealId={meal.id} />


{/each} 