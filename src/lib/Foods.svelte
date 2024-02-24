<script>
	// This displays all foods stored in their normalized format in the database.
	import { invoke } from '@tauri-apps/api/tauri';
	import { confirm } from '@tauri-apps/api/dialog';
	import { onMount } from 'svelte';
	import { foodsNormalized } from './store.js';
	import { _ } from 'svelte-i18n'; 

	// components
	import NewFood from './NewFood.svelte';
	import LoadCsv from './LoadCsv.svelte';
	import SaveCsv from './SaveCsv.svelte';
	import SingleFood from './SingleFood.svelte';

	async function refreshFoods() {
		foodsNormalized.set(await invoke('get_foods_normalized'));
		// sort by creation (id) in descending order
		$foodsNormalized.sort((a, b) => b.id - a.id);
	}

	onMount(async () => {
		await refreshFoods();
	});

	async function deleteFood(food) {
		const confirmed = await confirm($_('foods.deleteDialog.message'), {
			title: $_('foods.deleteDialog.title'),
			type: 'info'
		});
		if (!confirmed) return;
		await invoke('delete_food_normalized', { food });
		// refresh list
		await refreshFoods();
	}
</script>

<div class="mx-4">
	<!-- Load and save buttons -->
	<div class="flex space-x-4">
		<LoadCsv onUpdate={refreshFoods} />
		<SaveCsv />
	</div>

	<div class="flex justify-center">
		<NewFood />
	</div>
	<br />

	<div class="grid grid-cols-1 sm:grid-cols-2 gap-6">
		{#each $foodsNormalized as foodNormalized (foodNormalized.id)}
			<SingleFood {foodNormalized} onDelete={deleteFood} />
		{/each}
	</div>
</div>
