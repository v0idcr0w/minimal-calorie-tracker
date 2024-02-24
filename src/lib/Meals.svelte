<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { confirm } from '@tauri-apps/api/dialog';
	import { onMount } from 'svelte';
	import { toTitleCase } from './titleCase.js';
	import { dailyTotals, today, logId } from './store.js';
	import { _ } from 'svelte-i18n'; 
	import SingleMeal from './SingleMeal.svelte';
	import MaterialFloatingLabel from './MaterialFloatingLabel.svelte';
	import SvgOk from './SvgOk.svelte';
	import SvgCancel from './SvgCancel.svelte';
	import SvgAdd from './SvgAdd.svelte';
	import SvgTrash from './SvgTrash.svelte';

	const todayFormatted = $today.toISOString().split('T')[0];

	dailyTotals.set({ calories: 0, protein: 0, carbohydrate: 0, fat: 0 });

	let meals = [];
	let newMeal = {};
	let mealIds = [];
	let createMealActive = false;

	async function refreshMeals() {
		meals = await invoke('get_meals_by_log_id', { logId: $logId });
		meals.sort((a, b) => new Date(b.entry_timestamp) - new Date(a.entry_timestamp));
		mealIds = meals.map((obj) => obj.id);
		newMeal = { id: 0, log_id: $logId, name: '', entry_timestamp: '' };
	}

	async function updateTotals() {
		dailyTotals.set(await invoke('compute_daily_macros', { mealIds }));
		await invoke('update_log_totals', { logId: $logId, dailyTotals: $dailyTotals });
	}

	async function addNewMeal(newMeal) {
		// to make it compatible with Rust NaiveDateTime, remove the last character that represents the timezone.
		newMeal.entry_timestamp = $today.toISOString().slice(0, 23);
		await invoke('add_new_meal', { newMeal });
		await refreshMeals();
		// reset the newMeal object
		newMeal = { id: 0, log_id: $logId, name: '', entry_timestamp: '' };
		// set the createMealActive to false to hide the input field
		createMealActive = false;
		// calling updateTotals is unnecessary because meals are initialized empty
	}

	async function deleteMeal(meal) {
		const confirmed = await confirm($_('foods.deleteDialog.message'), {
			title: $_('foods.deleteDialog.title'),
			type: 'info'
		});
		if (!confirmed) return;
		await invoke('delete_meal', { meal });
		await refreshMeals();
		await updateTotals();
	}

	onMount(async () => {
		if (!$logId) {
			logId.set(await invoke('get_todays_log').id);
		}
		await refreshMeals();
		await updateTotals();
	});
</script>

<div class="mx-4">
	<div class="flex flex-col items-center">
		<!-- Meal creation -->
		<div class="mb-4">
			<button class="text-button" on:click={() => (createMealActive = !createMealActive)}>
				{#if !createMealActive}
					<SvgAdd /> {$_('meals.newMeal')}
				{:else}
					<SvgCancel /> {$_('cancel')}
				{/if}
			</button>
		</div>

		{#if createMealActive}
			<MaterialFloatingLabel label="Meal type" bind:value={newMeal.name} />
			<div class="mb-4">
				<button class="text-button" on:click={addNewMeal(newMeal)}>
					<SvgOk /> OK
				</button>
			</div>
		{/if}
	</div>

	<table class="mx-auto tracking-tighter text-sm mb-2">
		<thead>
			<tr>
				<th colspan="3">{$_('meals.total')} {todayFormatted}</th>
			</tr>
		</thead>
		<tr>
			<td>{$_('calories')}</td>
			<td>{$dailyTotals.calories.toFixed(1)}</td>
			<td>kcal</td>
		</tr>
		<tr>
			<td>{$_('protein')}</td>
			<td>{$dailyTotals.protein.toFixed(1)}</td>
			<td>g</td>
		</tr>
		<tr>
			<td>{$_('carbohydrates')}</td>
			<td>{$dailyTotals.carbohydrate.toFixed(1)}</td>
			<td>g</td>
		</tr>
		<tr>
			<td>{$_('fats')}</td>
			<td>{$dailyTotals.fat.toFixed(1)}</td>
			<td>g</td>
		</tr>
	</table>

	<div class="grid grid-cols-1 sm:grid-cols-2 gap-6">
		{#each meals as meal, index (meal.id)}
			<div
				class="block w-full text-center tracking-tighter rounded-lg bg-white p-2 shadow-[0_2px_15px_-3px_rgba(0,0,0,0.07),0_10px_20px_-2px_rgba(0,0,0,0.04)]"
			>
				<!-- Delete meal -->
				<h3 class="text-neutral-700 text-xl m-4 font-bold">{toTitleCase(meal.name)}</h3>
				<button class="icon-button mb-4" on:click={deleteMeal(meal)}>
					<SvgTrash />
				</button>

				<!-- render all foods associated with this meal -->
				<SingleMeal mealId={meal.id} onUpdate={updateTotals} />
			</div>
		{/each}
	</div>
</div>
