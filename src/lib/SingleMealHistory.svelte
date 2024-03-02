<script>
    import { invoke } from '@tauri-apps/api'; 
    import { onMount } from 'svelte'; 
    import { _ } from 'svelte-i18n';
    import { toTitleCase } from './titleCase';
    import { formatDate } from './formatDate';
    import Toggle from './Toggle.svelte'; 
    import SvgTrash from './SvgTrash.svelte'; 

    export let meal; 
    export let refreshMeals; 

    let foods = [];
    let mealMacros = {protein: 0, carbohydrate: 0, fat: 0, calories: 0}; 
    const date = new Date(meal.entry_timestamp); 

    async function getFoods() {
        foods = await invoke('get_foods_by_meal_id', { mealId: meal.id }); 
        mealMacros = await invoke('compute_meal_macros', { mealId: meal.id });
    }

    async function deleteMeal(meal) {
        await invoke('delete_meal', { meal }); 
        await refreshMeals(); 
    }

    async function handleToggle(meal) {
        meal.is_constant = !meal.is_constant;
		await invoke('update_meal_status', { meal });
    }


    onMount(async () => {
        await getFoods(); 
    });



</script>


<div>
    <Toggle isChecked={meal.is_constant} handleToggle={() => handleToggle(meal)} tooltipText={$_('meal_history.constant')} />
    
    <p class="text-neutral-700 text-xl mt-4 font-bold">{toTitleCase(meal.name)}</p>
    <p class="text-neutral-700 m-2">{$_('meal_history.created')} {formatDate(date)}</p>

    <button class="icon-button mb-4" on:click={deleteMeal(meal)}>
        <SvgTrash />
    </button>
    
    <ul class="w-full text-sm font-medium text-gray-900 bg-white border border-gray-200 rounded-lg">
        {#each foods as food (food.id)}
        <li
        class="w-full px-2 py-2 border-b border-gray-200 rounded-t-lg flex items-center justify-center">
        {toTitleCase(food.name)} {food.amount} {food.unit}</li>
        {/each}
    </ul>

    <p class="text-sm font-bold m-1">{$_('meal_history.nutrient')}</p>
    <table class="mx-auto text-sm">
        <tr>
            <td>{$_('calories')}</td>
            <td>{mealMacros.calories.toFixed(0)}</td>
            <td>kcal</td>
        </tr>
        <tr>
            <td>{$_('protein')}</td>
            <td> {mealMacros.protein.toFixed(1)}</td>
            <td>g</td>
        </tr>
        <tr>
            <td>{$_('carbohydrates')}</td>
            <td> {mealMacros.carbohydrate.toFixed(1)}</td>
            <td>g</td>
        </tr>
        <tr>
            <td>{$_('fats')}</td>
            <td> {mealMacros.fat.toFixed(1)}</td>
            <td>g</td>
        </tr>
    </table>
</div>