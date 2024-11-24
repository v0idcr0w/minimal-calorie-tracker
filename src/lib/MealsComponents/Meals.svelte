<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { confirm } from '@tauri-apps/api/dialog';
	import { onMount } from 'svelte';
	import { today, logId } from '$lib/store.js';
	import { _ } from 'svelte-i18n'; 
	import { foodsNormalized, recipes} from '$lib/store.js';
	import SingleMeal from './SingleMeal.svelte';
	// Components 
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from "$lib/components/ui/input";
	import { Label } from "$lib/components/ui/label";
	import * as Table from "$lib/components/ui/table";

	// Svg icons
	import { Plus } from 'lucide-svelte';

	// State  
	let dialogOpen = false; 
	let meals = [];
	let constMeals = []; 
	let newMeal = {};
	let dailyTotals = { calories: 0, protein: 0, carbohydrate: 0, fat: 0 };

	function resetNewMeal() {
		newMeal = { id: 0, log_id: $logId, name: '', entry_timestamp: '', is_constant: false, is_disabled: false};
	}

	async function refreshMeals() {
		// retrieve all meals by the today's log id and sort them by the entry timestamp
		meals = await invoke('get_meals_by_log_id', { logId: $logId });
		constMeals = await invoke('get_constant_meals', { logId: $logId });
		meals = meals.concat(constMeals); 
		meals.sort((a, b) => new Date(b.entry_timestamp) - new Date(a.entry_timestamp)); 
		// set the newMeal object
		resetNewMeal();
	}

	function updateMeal(event) {
		const updatedMeal = event.detail; 
		meals = meals.map((meal) => meal.id === updatedMeal.id ? updatedMeal : meal);
	}

	function filterMeals() {
		meals = meals.filter((meal) => meal.is_constant || meal.log_id === $logId);
	}

	async function updateTotals() {
		const activeMealIds = meals.filter((meal) => !meal.is_disabled).map((meal) => meal.id);
		dailyTotals = await invoke('compute_daily_macros', { mealIds: activeMealIds });
		await invoke('update_log_totals', { logId: $logId, dailyTotals });
	}

	async function addNewMeal() {
		// to make it compatible with Rust NaiveDateTime, remove the last character that represents the timezone.
		newMeal.entry_timestamp = $today.toISOString().slice(0, 23);
		newMeal.name = newMeal.name ? newMeal.name : 'Untitled';
		try {
			await invoke('add_new_meal', { newMeal });
			await refreshMeals();
			// reset the newMeal object and close the dialog
			resetNewMeal();
			dialogOpen = false; 
		} catch(error) {
			console.log(error); 
		}
	}

	async function deleteMeal(event) {
		const confirmed = await confirm($_('foods.deleteDialog.message'), {
			title: $_('foods.deleteDialog.title'),
			type: 'info'
		});
		if (!confirmed) return;
		const meal = event.detail; 
		try {
			await invoke('delete_meal', { meal });
			meals = meals.filter((m) => m.id !== meal.id);
			await updateTotals();
		} catch(error) {
			console.log(error); 
		}
	}

	onMount(async () => {
		if (!$logId) {
			logId.set(await invoke('get_todays_log').id);
		}
		recipes.set(await invoke('get_all_recipes'));
		foodsNormalized.set(await invoke('get_foods_normalized'));
		await refreshMeals();
		await updateTotals(); 
	});


</script>

<div class="mx-4">
	<!-- ! TABLE W/ TOTAL CALORIE AND MACRO CONSUMPTION -->
		<Table.Root class="mb-4 w-1/2 mx-auto">
			<Table.Caption class="caption-top mt-0">Today's Calorie and Macros Totals</Table.Caption>
			<Table.Row>
				<Table.Head>Calories</Table.Head>
				<Table.Cell>{dailyTotals.calories.toFixed(0)}</Table.Cell>
				<Table.Cell>kcal</Table.Cell>
			</Table.Row>
			<Table.Row>
				<Table.Head>Protein</Table.Head>
				<Table.Cell>{dailyTotals.protein.toFixed(1)}</Table.Cell>
				<Table.Cell>g</Table.Cell>
			</Table.Row>
			<Table.Row>
				<Table.Head>Carbohydrate</Table.Head>
				<Table.Cell>{dailyTotals.carbohydrate.toFixed(1)}</Table.Cell>
				<Table.Cell>g</Table.Cell>
			</Table.Row>
			<Table.Row>
				<Table.Head>Total Fat</Table.Head>
				<Table.Cell>{dailyTotals.fat.toFixed(1)}</Table.Cell>
				<Table.Cell>g</Table.Cell>
			</Table.Row>
		</Table.Root>
	<!-- ! CREATE MEAL BUTTON -->
	<div class="flex justify-center">
		<Button on:click={() => dialogOpen = true} class="mb-4">
			<Plus class="mr-2 h-4 w-4"/>
			{$_('meals.newMeal')}
		</Button>
		<Dialog.Root bind:open={dialogOpen}>
			<Dialog.Trigger /> 
			<Dialog.Content>
				<Dialog.Header>
					<Dialog.Title>
						Create New Meal
					</Dialog.Title>
				</Dialog.Header>
				<div class="grid grid-cols-4 gap-2 items-center">
					<Label for="name" class="text-right">Meal Name</Label>
					<Input id="name" placeholder="Breakfast" class="col-span-3" bind:value={newMeal.name}/>
				</div>
				<Dialog.Footer>
					<Button type="submit" on:click={addNewMeal}>Confirm</Button>
					<Dialog.Close>
						<Button>Close</Button>
					</Dialog.Close>
				</Dialog.Footer>
			</Dialog.Content>
		</Dialog.Root>
	</div>

	<!-- ! RENDERING SINGLE MEALS -->
	<div class="grid grid-cols-2 xl:grid-cols-4 lg:grid-cols-3 portrait:grid-cols-1 gap-2">
		{#each meals as meal (meal.id)}
			<SingleMeal {meal} on:refresh={filterMeals} on:update={updateMeal} on:updateTotals={updateTotals} on:delete={deleteMeal} />
		{/each}
	</div>
</div>
