<script>
	// This displays all foods stored in their normalized format in the database.
	import { invoke } from '@tauri-apps/api/tauri';
	import { confirm } from '@tauri-apps/api/dialog';
	import { onMount } from 'svelte';
	import { foodsNormalized } from '../store.js';
	import { _ } from 'svelte-i18n'; 
	import { Button } from '$lib/components/ui/button'; 
	import { Plus } from 'lucide-svelte'; 
	import * as Dialog from "$lib/components/ui/dialog"; 
	import { Input } from "$lib/components/ui/input";
	import { Label } from "$lib/components/ui/label";

	// components
	import LoadCsv from './LoadCsv.svelte';
	import SaveCsv from './SaveCsv.svelte';
	import SingleFood from './SingleFood.svelte';
	import { validateNumber } from '$lib/appUtils.js';

	let dialogOpen = false; 
	let newFood = {
		id: 0,
		name: '',
		serving_size: null, // number 
		unit: '',
		normalized_calories: null, // number
		normalized_protein: null, // number
		normalized_carbohydrate: null, // number 
		normalized_fat: null // number 
	};
	let validationError = {}; 

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
		try {
			await invoke('delete_food_normalized', { food });	
			// if no error returned, filter out the deleted foods
			foodsNormalized.update((foods) => foods.filter((f) => f.id !== food.id));
		} catch (error) {
			console.error(error);
		}
	}

	function resetNewFood() {
		newFood = {
			id: 0,
			name: '',
			serving_size: null, // number 
			unit: '',
			normalized_calories: null, // number
			normalized_protein: null, // number
			normalized_carbohydrate: null, // number 
			normalized_fat: null // number 
		};
	}

	function validateWithDefaults() {
		if (!newFood.name.trim()) {
			newFood.name = "Unnamed"
		}
		if (!validateNumber(newFood.serving_size)) {
			newFood.serving_size = 100; 
		}
		if (!newFood.unit) {
			newFood.unit = "g";
		}
		if (!validateNumber(newFood.normalized_protein)) {
			newFood.normalized_protein = 0;
		}
		if (!validateNumber(newFood.normalized_carbohydrate)) {
			newFood.normalized_carbohydrate = 0;
		}
		if (!validateNumber(newFood.normalized_fat)) {
			newFood.normalized_fat = 0; 
		}
		if (!validateNumber(newFood.normalized_calories)) {
			newFood.normalized_calories = 0; 
		}
	}

	async function addFood() {
		validateWithDefaults();
		try {
			const addedFood = await invoke('add_new_food_normalized', { 
				newFood: {
					...newFood, 
					serving_size: Number(newFood.serving_size), 
					normalized_calories: Number(newFood.normalized_calories), 
					normalized_protein: Number(newFood.normalized_protein), 
					normalized_carbohydrate: Number(newFood.normalized_carbohydrate), 
					normalized_fat: Number(newFood.normalized_fat)
				}
			 }); 
			foodsNormalized.update((foods) => [addedFood, ...foods]); 	
			resetNewFood(); 
			dialogOpen = false; // close the modal 
		} catch (error) {
			console.log(error); 
		}
	}

</script>

<div class="mx-4">
	<!-- Load and save buttons -->
	<div class="flex space-x-4">
		<LoadCsv onUpdate={refreshFoods} />
		<SaveCsv />
	</div>

	<div class="flex justify-center">
		<Button on:click={() => dialogOpen = true} class="mb-4">
			<Plus class="mr-2 h-4 w-4"/>
			{$_('foods.newFood')}
		</Button>
		<Dialog.Root bind:open={dialogOpen}>
			<Dialog.Trigger />
			<Dialog.Content>
				<Dialog.Header>
					<Dialog.Title>Add New Food</Dialog.Title>
				</Dialog.Header>
				<div class="grid gap-4 py-4">
					<div class="grid grid-cols-4 items-center gap-2">
						<Label for="name" class="text-right">Food Name</Label>
						<Input id="name" placeholder="Apple" class="col-span-3" bind:value={newFood.name} />
						<Label for="servingSize" class="text-right">Serving Size</Label>
						<Input type="number" id="servingSize" placeholder="100" class="col-span-1" bind:value={newFood.serving_size} />
						<Label for="units"  class="text-right col-span-1">Units</Label>
						<Input id="units" placeholder="g" class="col-span-1" bind:value={newFood.unit} />
						<Label for="calories" class="text-right">Calories (kcal)</Label>
						<Input type="number" id="calories" class="col-span-1" bind:value={newFood.normalized_calories} />
						<Label for="protein" class="text-right">Protein (g)</Label>
						<Input type="number" id="protein" class="col-span-1" bind:value={newFood.normalized_protein} />
						<Label for="carbs" class="text-right">Carbohydrate (g)</Label>
						<Input type="number" id="carbs" class="col-span-1" bind:value={newFood.normalized_carbohydrate} />
						<Label for="fat" class="text-right">Fat (g)</Label>
						<Input type="number" id="fat" class="col-span-1" bind:value={newFood.normalized_fat} />
					</div>
				  </div>
				  <Dialog.Footer>
					<Button type="submit" on:click={addFood}>Confirm</Button>
					<Dialog.Close>
						<Button>
							Cancel
						</Button>
					</Dialog.Close>
				  </Dialog.Footer>
			</Dialog.Content>
		</Dialog.Root>
	</div>
	
	<div class="grid grid-cols-2 xl:grid-cols-4 lg:grid-cols-3 portrait:grid-cols-1 gap-2">
		{#each $foodsNormalized as foodNormalized (foodNormalized.id)}
			<SingleFood {foodNormalized} onDelete={deleteFood} />
		{/each}
	</div>
</div>
