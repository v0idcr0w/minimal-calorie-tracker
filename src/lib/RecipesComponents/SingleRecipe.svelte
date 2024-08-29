<script>
	import SingleIngredient from './SingleIngredient.svelte';
	import { recipes } from '../store.js'; 
	import { invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';
	import { foodsNormalized } from '../store.js';
	import { _ } from 'svelte-i18n'; 


	// Components
	import * as Card from '$lib/components/ui/card';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from "$lib/components/ui/dialog"; 
	import * as Table from "$lib/components/ui/table";
	import * as Select from '$lib/components/ui/select';

	// Svg icons
	import { PencilLine } from 'lucide-svelte';
	import { Plus } from 'lucide-svelte'; 
	import { X } from 'lucide-svelte'; 

	// props
	export let recipe;
	export let onDelete;

	// state
	let editModalOpen = false; 
	let addIngredientModalOpen = false; 
	let recipeCopy = { ...recipe };
	let ingredients = []; // all ingredients
	let selectedIngredients = []; 

	async function editRecipe() {
		try {
			let updatedRecipe = await invoke('update_recipe', { recipe: {
				...recipe,
				serving_size: Number(recipe.serving_size)
			}});
			recipes.update((recipes) => recipes.map((r) => (r.id === updatedRecipe.id ? updatedRecipe : r)));
			recipeCopy = { ...updatedRecipe };
			editModalOpen = false;
		} catch(error) {
			console.log(error); 
		}
	}

	async function refreshIngredients() {
		ingredients = await invoke('get_ingredients_by_recipe_id', { recipeId: recipe.id });
	}

	onMount(async () => {
		await refreshIngredients();
	});


	async function addIngredientToRecipe() {
		const toAdd = selectedIngredients.map((s) => $foodsNormalized.find((f) => f.id === s.value));
		try {
			// get ingredients and add them
			const addedIngredients = await invoke('add_ingredient_to_recipe', {ingredientList: toAdd, recipeId: recipe.id});
			ingredients = [...ingredients, ...addedIngredients]; 
			addIngredientModalOpen = false; 
		} catch (error) {
			console.log(error); 
		}
	}

	async function deleteIngredientFromRecipe(event) {
		event.preventDefault(); 
		const ingredient = event.detail; 
		try {
			recipe = await invoke('delete_ingredient_from_recipe', { ingredient });
			refreshIngredients(); 
		} catch (error) {
			console.log(error);
		}
	}

	async function updateIngredientAmount(event) {
		// fetch new recipe with update macros
		// update in the UI
		event.preventDefault(); 
		const { ingredient, newAmount } = event.detail;
		try {
			recipe = await invoke('update_ingredient_amount', { ingredient, newAmount });
			ingredients = ingredients.map(i => {
			if (i.id === ingredient.id) {
				return { ...i, amount: newAmount };
			}
			return i;
			});
		} catch(error) {
			console.log(error); 
		}
	}
</script>

<!-- *************************** -->
<!-- *************************** -->
<!-- *************************** -->

<Card.Root>
	<div class="flex w-full justify-between">
		<Button class="rounded-none" variant="ghost" size="icon" on:click={() => editModalOpen = true}>
			<PencilLine class="h-4 w-4"/>
		</Button>
		<Button class="ml-auto rounded-none" variant="ghost" size="icon" on:click={() => onDelete(recipe.id)}>
			<X class="h-4 w-4"/>
		</Button>
	</div>
	<Card.Header class="pt-0 pb-2">
		<Card.Title class="capitalize">
			{recipe.name} 
		</Card.Title>
		<Card.Description>
			{recipe.serving_size} {recipe.unit}
		</Card.Description>
	</Card.Header>
	<Card.Content>
		<div class="flex flex-col gap-2 justify-center">

		<!-- ! ADD INGREDIENT BUTTONE -->
		<!-- List with multiple selectable elements; once one gets clicked,  calls backend inserting the ingredient amount to 0 by default; the user can later edit the amount inside the table that gets rendered containing each ingredient and its amount.  -->
		<Button on:click={() => addIngredientModalOpen = true}>
			<Plus class="mr-2 h-4 w-4"/>
			Add Ingredient
		</Button>
		<Dialog.Root bind:open={addIngredientModalOpen}>
			<Dialog.Trigger/>
			<Dialog.Content>
				<Dialog.Header>
					<Dialog.Title>Add Ingredients</Dialog.Title>
					<Dialog.Description>
						Select one or more ingredients to add to the <span class="capitalize">{recipe.name}</span> recipe.
					</Dialog.Description>
				</Dialog.Header>
				<div>
					<Select.Root multiple selected={selectedIngredients} onSelectedChange={(s) => selectedIngredients =s }>
						<Select.Trigger>
							<Select.Value placeholder="Ingredients"/>
						</Select.Trigger>
						<Select.Content>
							{#each $foodsNormalized as food}
								<Select.Item value={food.id} label={food.name} class="capitalize"/>
							{/each}
						</Select.Content>
					</Select.Root>
				</div>
				<Dialog.Footer>
					<Button on:click={() => addIngredientToRecipe()}>Confirm</Button>
					<Dialog.Close>
						<Button on:click={() => selectedIngredients = []}>
							Cancel
						</Button>
					</Dialog.Close>
				</Dialog.Footer>
			</Dialog.Content>
		</Dialog.Root>
		
		<!-- ! DISPLAY THE INGREDIENTS -->
		<div class="grid grid-cols-1 gap-2">
			{#each ingredients as ingredient (ingredient.id)}
			<SingleIngredient  {ingredient} on:delete={deleteIngredientFromRecipe} on:update={updateIngredientAmount} />
			{/each}
		</div>
		</div>

		<!-- ! TOTAL NUTRITION FACTS BELOW -->
		<Table.Root>
			<Table.Caption class="caption-top">Nutrition Facts</Table.Caption>
			<Table.Row>
				<Table.Head>Calories</Table.Head>
				<Table.Cell class="text-right">{recipe.calories.toFixed(0)}</Table.Cell>
				<Table.Cell class="text-left">kcal</Table.Cell>
			</Table.Row>
			<Table.Row>
				<Table.Head>Protein</Table.Head>
				<Table.Cell class="text-right">{recipe.protein.toFixed(1)}</Table.Cell>
				<Table.Cell class="text-left">g</Table.Cell>
			</Table.Row>
			<Table.Row>
				<Table.Head>Carbohydrate</Table.Head>
				<Table.Cell class="text-right">{recipe.carbohydrate.toFixed(1)}</Table.Cell>
				<Table.Cell class="text-left">g</Table.Cell>
			</Table.Row>
			<Table.Row>
				<Table.Head>Total Fat</Table.Head>
				<Table.Cell class="text-right">{recipe.fat.toFixed(1)}</Table.Cell>
				<Table.Cell class="text-left">g</Table.Cell>
			</Table.Row>
		</Table.Root>
	</Card.Content>
	<Card.Footer>
		<div class="flex w-full justify-between">
			<!-- ? EDITING MODAL -->
			<Dialog.Root bind:open={editModalOpen}>
				<Dialog.Trigger />
				<Dialog.Content>
					<Dialog.Header>
						<Dialog.Title>Edit Recipe</Dialog.Title>
					</Dialog.Header>
					<div class="grid gap-4 py-4">
						<div class="grid grid-cols-4 items-center gap-2">
							<Label for="editRecipeName" class="text-right col-span-1">Recipe Name</Label>
							<Input id="editRecipeName" class="col-span-3" bind:value={recipe.name}/>
							<Label for="editServingSize" class="text-right col-span-1">Serving Size</Label>
							<Input id="editServingSize" class="col-span-1" bind:value={recipe.serving_size}/>
							<Label for="editUnit" class="text-right col-span-1">Units</Label>
							<Input id="editUnit" class="col-span-1" bind:value={recipe.unit}/>
						</div>
					</div>
					<Dialog.Footer>
						<Button type="submit" on:click={editRecipe}>Confirm</Button>
						<Dialog.Close>
							<!-- Restore the recipe object -->
							<Button on:click={() => recipe = {...recipeCopy}}>
								Cancel
							</Button>
						</Dialog.Close>
					</Dialog.Footer>
				</Dialog.Content>		
			</Dialog.Root>
		</div>
	</Card.Footer>
</Card.Root>
