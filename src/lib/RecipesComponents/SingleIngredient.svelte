<script>
	import { invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';
	import { foodsNormalized } from '../store.js';
	import { toTitleCase } from '../titleCase.js';
	import { _ } from 'svelte-i18n'; 
	import MaterialFloatingLabel from '../MaterialFloatingLabel.svelte';
	import EditableField from '../EditableField.svelte';
	import SvgOk from '../SvgOk.svelte';
	import SvgEdit from '../SvgEdit.svelte';
	import SvgCancel from '../SvgCancel.svelte';
	import SvgAdd from '../SvgAdd.svelte';
	import SvgTrash from '../SvgTrash.svelte';
	import SvgRemove from '../SvgRemove.svelte';
	import GradientButton from '../GradientButton.svelte';

	// Components
	import { Button } from '$lib/components/ui/button';

	// Svg icons
	import { Trash2 } from 'lucide-svelte'; 
	import { Check } from 'lucide-svelte'; 
	import { PencilLine } from 'lucide-svelte';
	import { PenOff } from 'lucide-svelte';
	import { TextCursorInput } from 'lucide-svelte'; 
	import { Plus } from 'lucide-svelte'; 
	import { X } from 'lucide-svelte'; 

	// props
	export let ingredient; 
    export let index; 
    export let newIngredientAmounts; 

	// ingredient variables
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

