<script>
    import { invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';
    import SingleMealHistory from './SingleMealHistory.svelte';

    // State
    let allMeals = []; 
    async function getMeals() {
        allMeals = await invoke('get_all_meals');
    }

    onMount(async () => {
        await getMeals();
    });

</script>

<div class="mx-4">
    <!-- cards -->
    <div class="grid grid-cols-2 xl:grid-cols-4 lg:grid-cols-3 portrait:grid-cols-1 gap-2">
		{#each allMeals as meal (meal.id)}
                <SingleMealHistory {meal} refreshMeals={getMeals} /> 
        {/each} 
	</div>

</div>