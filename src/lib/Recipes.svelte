<script>
	import { invoke } from '@tauri-apps/api';
	import { confirm } from '@tauri-apps/api/dialog';
	import { onMount } from 'svelte';
	import { logId, foodsNormalized, recipes } from './store.js';
	import { _ } from 'svelte-i18n'; 
	import SingleRecipe from './SingleRecipe.svelte';
	import MaterialFloatingLabel from './MaterialFloatingLabel.svelte';
	import SvgOk from './SvgOk.svelte';
	import SvgCancel from './SvgCancel.svelte';
	import SvgAdd from './SvgAdd.svelte';

	let newRecipe = { name: '', serving_size: 0.0, unit: '' };
	let newRecipeActive = false;
	// Check if the new recipe box is properly filled
	$: inputsFilled = newRecipe.name !== '' && newRecipe.serving_size >= 0.0 && newRecipe.unit !== '';

	onMount(async () => {
		if ($foodsNormalized.length === 0) {
			foodsNormalized.set(await invoke('get_foods_normalized'));
		}
		if (!$logId) {
			logId.set(await invoke('get_todays_log').id);
		}
		await getAllRecipes();
	});

	async function createNewRecipe() {
		await invoke('create_new_recipe', {
			name: newRecipe.name,
			servingSize: newRecipe.serving_size,
			unit: newRecipe.unit
		});
		// reset button status
		newRecipeActive = false;
		// reset newRecipe object
		newRecipe = { name: '', serving_size: 0.0, unit: '' };
		// refresh recipes
		await getAllRecipes();
	}

	async function getAllRecipes() {
		recipes.set(await invoke('get_all_recipes'));
		// sort by descending order of id
		$recipes.sort((a, b) => b.id - a.id);
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
	<!-- Creating a new recipe -->
	<div class="flex flex-col items-center justify-center">
		<div class="mb-4">
			<div class="flex justify-center">
				<button class="text-button mb-4" on:click={() => (newRecipeActive = !newRecipeActive)}>
					{#if !newRecipeActive}
						<SvgAdd /> {$_('recipes.newRecipe')}
					{:else}
						<SvgCancel /> {$_('cancel')}
					{/if}
				</button>
			</div>

			{#if newRecipeActive}
				<div class="block tracking-tighter text-sm">
					<table class="mx-auto">
						<tr>
							<td colspan="2">
								<MaterialFloatingLabel
									label={$_('recipes.recipeName')}
									bind:value={newRecipe.name}
									type="text"
								/>
							</td>
						</tr>
						<tr>
							<td
								><MaterialFloatingLabel
									label={$_('servingSize')}
									bind:value={newRecipe.serving_size}
									type="number"
								/></td
							>
							<td>
								<MaterialFloatingLabel
									label={$_('unit')}
									bind:value={newRecipe.unit}
									type="text"
								/>
							</td>
						</tr>
					</table>

					<!-- Flex justify-center is necessary to center these buttons  -->
					<div class="flex justify-center">
						<button class="text-button" on:click={createNewRecipe} disabled={!inputsFilled}>
							<SvgOk /> OK
						</button>
					</div>
				</div>
			{/if}
		</div>
	</div>

	<!-- Rendering all recipes below -->
	<div class="grid grid-cols-1 sm:grid-cols-2 gap-6">
		{#each $recipes as recipe (recipe.id)}
			<SingleRecipe {recipe} onDelete={() => deleteRecipe(recipe.id)} />
		{/each}
	</div>
</div>
