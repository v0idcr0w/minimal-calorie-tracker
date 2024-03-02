<script>
	import { invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';
	import { foodsNormalized } from './store.js';
	import { toTitleCase } from './titleCase.js';
	import { _ } from 'svelte-i18n'; 
	import MaterialFloatingLabel from './MaterialFloatingLabel.svelte';
	import EditableField from './EditableField.svelte';
	import SvgOk from './SvgOk.svelte';
	import SvgEdit from './SvgEdit.svelte';
	import SvgCancel from './SvgCancel.svelte';
	import SvgAdd from './SvgAdd.svelte';
	import SvgTrash from './SvgTrash.svelte';
	import SvgRemove from './SvgRemove.svelte';
	import GradientButton from './GradientButton.svelte';
	// props
	export let recipe;
	export let onDelete;

	// ingredient variables
	let ingredients = [];
	let editIngredientFlags = [];
	let newIngredientAmounts = [];

	// dropdown selection variables
	let dropdownActive = false;
	let selectedFoodId = 0;
	let selectedFoodAmount = 0;
	let selectedFood = {};

	//  editing serving size variables
	let editServingSizeActive = false;
	let newServingSize = 0;
	let newUnit = recipe.unit;

	onMount(async () => {
		await refreshIngredients();
	});

	async function refreshIngredients() {
		ingredients = await invoke('get_ingredients_by_recipe_id', { recipeId: recipe.id });
		editIngredientFlags = new Array(ingredients.length).fill(false);
		newIngredientAmounts = ingredients.map((ingredient) => ingredient.amount);
	}

	async function renameRecipe(obj, text) {
		await invoke('update_recipe_name', { recipeId: obj.id, newName: text });
	}

	async function updateServingSize(newServingSize, newUnit) {
		recipe = await invoke('update_recipe_serving_size', { recipe, newServingSize, newUnit });
		editServingSizeActive = false;
	}

	function handleSelect(event) {
		// handles selection in dropdown menu to add new ingredient
		const selectedId = event.target.value; // value is a string
		selectedFoodId = selectedId;
		selectedFood = $foodsNormalized.find((item) => item.id == selectedId);
	}

	async function addIngredientToRecipe(selectedFood, selectedFoodAmount) {
		recipe = await invoke('add_ingredient_to_recipe', {
			foodNormalized: selectedFood,
			recipeId: recipe.id,
			amount: Number(selectedFoodAmount)
		});
		// refresh ui
		dropdownActive = false;
		refreshIngredients();
		selectedFoodAmount = 0;
	}

	async function deleteIngredientFromRecipe(ingredient) {
		recipe = await invoke('delete_ingredient_from_recipe', { ingredient });
		// refresh the list of ingredients
		refreshIngredients();
	}

	async function updateIngredientAmount(ingredient, newAmount) {
		// fetch new recipe with update macros
		// update in the UI
		recipe = await invoke('update_ingredient_amount', { ingredient, newAmount });
		ingredient.amount = newAmount;
		// reset all flags to false
		editIngredientFlags = new Array(ingredients.length).fill(false);
	}
</script>

<!-- *************************** -->
<!-- *************************** -->
<!-- *************************** -->

<div
	class="block w-full tracking-tighter rounded-lg bg-white p-2 shadow-[0_2px_15px_-3px_rgba(0,0,0,0.07),0_10px_20px_-2px_rgba(0,0,0,0.04)]"
>
	<!-- new rename -->
	<div class="block w-full text-center tracking-tighter ">
	<EditableField text={toTitleCase(recipe.name)} handleRename={renameRecipe} obj={recipe} />
	</div>
	
	<!-- Delete button -->
	<div class="flex justify-center">
		<button class="icon-button mb-2 mx-2" on:click={onDelete}>
			<SvgTrash />
		</button>
	</div>

	<!-- RENDERING / EDITING SERVING SIZE -->
	<div class="flex items-center justify-center my-2">
		{#if !editServingSizeActive}
			<p class="text-center text-sm mx-2">{$_('servingSize')} {recipe.serving_size} {recipe.unit}</p>
		{/if}
		{#if !editServingSizeActive}
			<GradientButton onClick={() => (editServingSizeActive = !editServingSizeActive)}>
				<SvgEdit />
			</GradientButton>
		{:else}
			<GradientButton onClick={() => (editServingSizeActive = !editServingSizeActive)}>
				<SvgCancel />
			</GradientButton>
		{/if}
	</div>

	{#if editServingSizeActive}
		<div class="flex">
			<div class="mr-2">
				<MaterialFloatingLabel label={$_('servingSize')} bind:value={newServingSize} type="number" />
			</div>
			<MaterialFloatingLabel label={$_('unit')} bind:value={newUnit} />
		</div>
		<div class="flex items-center justify-center">
			<GradientButton
				onClick={() => updateServingSize(newServingSize, newUnit)}
				disabled={!(newServingSize > 0 && newUnit)}
			>
				<SvgOk />
			</GradientButton>
		</div>
	{/if}

	<!-- RENDERING / EDITING INGREDIENT LIST -->

	<ul class="w-full text-sm font-medium text-gray-900 bg-white border border-gray-200 rounded-lg">
		{#each ingredients as ingredient, index (ingredient.id)}
			<li
				class="w-full px-4 py-2 border-b border-gray-200 rounded-t-lg flex items-center justify-between text-left"
			>
				<div class="-me-2 -mb-2">
					<GradientButton onClick={() => deleteIngredientFromRecipe(ingredient)}>
						<SvgRemove />
					</GradientButton>
				</div>

				<div class="flex items-center">
					<span class="inline-block mx-1 align-top">
						{toTitleCase(ingredient.name)}
					</span>

					{#if editIngredientFlags[index]}
						<input
							class="w-8 text-center inline-block my-2 align-top"
							type="number"
							min="0"
							placeholder="0"
							bind:value={newIngredientAmounts[index]}
						/>
					{:else}
						<span class="inline-block my-2 align-top">
							{ingredient.amount}
						</span>
					{/if}

					<span class="inline-block my-2 mx-1 align-top">
						{ingredient.unit}
					</span>
				</div>

				<div class="-mb-2 flex items-center">
					{#if !editIngredientFlags[index]}
						<GradientButton
							onClick={() => (editIngredientFlags[index] = !editIngredientFlags[index])}
						>
							<SvgEdit />
						</GradientButton>
					{:else}
						<GradientButton
							onClick={() => (editIngredientFlags[index] = !editIngredientFlags[index])}
						>
							<SvgCancel />
						</GradientButton>
					{/if}

					<div class="-me-2">
						{#if editIngredientFlags[index]}
							<GradientButton
								onClick={() => updateIngredientAmount(ingredient, newIngredientAmounts[index])}
								disabled={!(newIngredientAmounts[index] >= 0)}
							>
								<SvgOk />
							</GradientButton>
						{/if}
					</div>
				</div>
			</li>
		{/each}
	</ul>

	<!-- Adding new ingredient block -->
	<div class="flex justify-center">
		<button class="icon-button my-2" on:click={() => (dropdownActive = !dropdownActive)}>
			{#if !dropdownActive}
				<SvgAdd />
			{:else}
				<SvgCancel />
			{/if}
		</button>

		{#if dropdownActive && selectedFoodId}
			<button
				class="icon-button my-2 mx-2"
				on:click={() => addIngredientToRecipe(selectedFood, selectedFoodAmount)}
				disabled={!(selectedFoodAmount >= 0)}
			>
				<SvgOk />
			</button>
		{/if}
	</div>

	{#if dropdownActive}
		<div class="flex justify-center">
			<select
				class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 w-1/2 p-1"
				id="foodDropdown"
				on:change={handleSelect}
			>
				<!-- first option is blank and unselectable -->
				<option value="" disabled selected hidden></option>
				{#each $foodsNormalized as foodNormalized (foodNormalized.id)}
					<option value={foodNormalized.id}>{toTitleCase(foodNormalized.name)}</option>
				{/each}
			</select>
		</div>
		<!-- Select the amount -->
		{#if selectedFoodId}
			<div class="flex flex-col items-center mt-4">
				<div class="block text-sm">
					<MaterialFloatingLabel
						label="{$_('amount')} ({selectedFood.unit})"
						bind:value={selectedFoodAmount}
						type="number"
					/>
				</div>
			</div>
		{/if}
	{/if}

	<!-- Recipe totals information -->
	<table class="mx-auto">
		<thead>
			<tr>
				<th colspan="3">{$_('total')}</th>
			</tr>
		</thead>
		<tr>
			<td>{$_('protein')}</td>
			<td>{recipe.protein.toFixed(1)}</td>
			<td>g</td>
		</tr>
		<tr>
			<td>{$_('carbohydrates')}</td>
			<td>{recipe.carbohydrate.toFixed(1)}</td>
			<td>g</td>
		</tr>
		<tr>
			<td>{$_('fats')}</td>
			<td>{recipe.fat.toFixed(1)}</td>
			<td>g</td>
		</tr>
		<tr>
			<td>{$_('calories')}</td>
			<td>{recipe.calories.toFixed(0)}</td>
			<td>kcal</td>
		</tr>
	</table>
</div>
