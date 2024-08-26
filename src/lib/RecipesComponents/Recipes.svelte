<script>
	import { invoke } from '@tauri-apps/api';
	import { confirm } from '@tauri-apps/api/dialog';
	import { onMount } from 'svelte';
	import { logId, foodsNormalized, recipes } from '../store.js';
	import { _ } from 'svelte-i18n'; 
	import SingleRecipe from './SingleRecipe.svelte';

	// Components
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from "$lib/components/ui/input";
	import { Label } from "$lib/components/ui/label";

	// Svg Icons
	import { Plus } from 'lucide-svelte'; 

	// State
	let dialogOpen = false; 
	let newRecipe = { name: '', serving_size: null, unit: '' };

	async function getAllRecipes() {
		recipes.set(await invoke('get_all_recipes'));
		// sort by descending order of id
		$recipes.sort((a, b) => b.id - a.id);
	}

	onMount(async () => {
		if ($foodsNormalized.length === 0) {
			foodsNormalized.set(await invoke('get_foods_normalized'));
		}
		if (!$logId) {
			logId.set(await invoke('get_todays_log').id);
		}
		await getAllRecipes();
	});

	// TODO: VALIDATION OF THIS FORM
	async function addRecipe() {
		try {
			const addedRecipe = await invoke('create_new_recipe', {
			name: newRecipe.name,
			servingSize: Number(newRecipe.serving_size),
			unit: newRecipe.unit
			});
			$recipes = [addedRecipe, ...$recipes];
			// close modal
			dialogOpen = false; 
			// reset newRecipe object
			newRecipe = { name: '', serving_size: null, unit: '' };
		} catch(error) {
			console.error(error);
		}
	}

	async function deleteRecipe(recipeId) {
		const confirmed = await confirm($_('foods.deleteDialog.message'), {
			title: $_('foods.deleteDialog.title'),
			type: 'info'
		});
		if (!confirmed) return;
		await invoke('delete_recipe', { recipeId });
		// refresh recipe list
		await getAllRecipes();
	}
</script>

<div class="mx-4">
	<!-- ! NEW RECIPE -->
	<div class="flex flex-col items-center justify-center">
		<Button class="mb-4" on:click={() => dialogOpen = true}>
			<Plus class="mr-2 h-4 w-4"/>
			New Recipe
		</Button>
		<!-- ? DIALOG -->
		 <Dialog.Root bind:open={dialogOpen}>
			<Dialog.Trigger/>
			<Dialog.Content>
				<Dialog.Header>
					Add New Recipe
				</Dialog.Header>
				<div class="grid gap-4 py-4">
					<div class="grid grid-cols-4 items-center gap-2">
						<Label for="recipeName" class="text-right col-span-1">Recipe Name</Label>
						<Input id="recipeName" class="col-span-3" bind:value={newRecipe.name}/>
						<Label for="servingSize" class="text-right col-span-1">Serving Size</Label>
						<Input id="servingSize" class="col-span-1" bind:value={newRecipe.serving_size}/>
						<Label for="unit" class="text-right col-span-1">Units</Label>
						<Input id="unit" class="col-span-1" bind:value={newRecipe.unit}/>
					</div>
				</div>
				<Dialog.Footer>
					<Button type="submit" on:click={addRecipe}>Confirm</Button>
					<Dialog.Close>
						<Button>
							Cancel
						</Button>
					</Dialog.Close>
				  </Dialog.Footer>
			</Dialog.Content>
		 </Dialog.Root>
</div>

	<!-- ! ALL RECIPES -->
	<div class="grid grid-cols-2 xl:grid-cols-4 lg:grid-cols-3 portrait:grid-cols-1 gap-2">
		{#each $recipes as recipe (recipe.id)}
			<SingleRecipe {recipe} onDelete={() => deleteRecipe(recipe.id)} />
		{/each}
	</div>
</div>