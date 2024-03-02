<script>
    import { invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';
    import SingleMealHistory from './SingleMealHistory.svelte';

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
    <div class="grid grid-cols-1 sm:grid-cols-2 gap-6">
		{#each allMeals as meal (meal.id)}
            <div
            class="block w-full text-center tracking-tighter rounded-lg bg-white p-2 shadow-[0_2px_15px_-3px_rgba(0,0,0,0.07),0_10px_20px_-2px_rgba(0,0,0,0.04)]"
            >	
                <SingleMealHistory {meal} refreshMeals={getMeals} /> 
            </div>
        {/each} 
	</div>

</div>