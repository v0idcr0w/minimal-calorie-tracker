<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { foodsNormalized } from '../store.js';
	import { _ } from 'svelte-i18n';

	// Svg icons for buttons
	import { Trash2 } from 'lucide-svelte'; 
	import { Check } from 'lucide-svelte'; 
	import { PencilLine } from 'lucide-svelte';
	import { PenOff } from 'lucide-svelte';
	import { TextCursorInput } from 'lucide-svelte'; 

	// Components
	import * as Card from '$lib/components/ui/card';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from "$lib/components/ui/dialog"; 

	// Props
	export let foodNormalized;
	export let onDelete;

	// State
	let editable = false; 
	let renameModalOpen = false; 

	async function updateFood() {
		try {
			foodNormalized = await invoke('update_food_normalized', { 'food': {...foodNormalized, 
				'serving_size': Number(foodNormalized.serving_size),
				'normalized_calories': Number(foodNormalized.normalized_calories),
				'normalized_protein': Number(foodNormalized.normalized_protein),
				'normalized_carbohydrate': Number(foodNormalized.normalized_carbohydrate),
				'normalized_fat': Number(foodNormalized.normalized_fat),
			} });
			foodsNormalized.update((foods) => foods.map((f) => (f.id === foodNormalized.id ? foodNormalized : f)));
		} catch (error)  {
			console.log(error); 
		}
		editable = false;
	}

	async function updateFoodName() {
		try {
			foodNormalized = await invoke('update_food_normalized_name', { id: foodNormalized.id, newName: foodNormalized.name });
			foodsNormalized.update((foods) => foods.map((f) => (f.id === foodNormalized.id ? foodNormalized : f)));
		} catch (error) {
			console.log(error); 
		}
		renameModalOpen = false; 
	}
</script>
<Card.Root>
	<Card.Header>
		<Card.Title class="capitalize">
			{foodNormalized.name}
		</Card.Title>
		<Card.Description>
			<Button variant="ghost" on:click={() => renameModalOpen = true}>
				<TextCursorInput class="mr-2 h-4 w-4"/>
				Rename
			</Button>
			<!-- ? RENAMING MODAL -->
			<Dialog.Root bind:open={renameModalOpen}>
				<Dialog.Trigger />
				<Dialog.Content>
					<Dialog.Header>
						<Dialog.Title>Rename Food</Dialog.Title>
					</Dialog.Header>
					<div>
						<div class="grid grid-cols-4 items-center gap-2">
							<Label for="name" class="text-right">New Name</Label>
							<Input id="name" placeholder="Apple" class="col-span-3" bind:value={foodNormalized.name} />
						</div>
					</div>
					<Dialog.Footer>
						<Button type="submit" on:click={updateFoodName}>Confirm</Button>
						<Dialog.Close>
							<Button>
								Cancel
							</Button>
						</Dialog.Close>
					  </Dialog.Footer>
				</Dialog.Content>
			</Dialog.Root>
			<!-- ? END OF RENAMING MODAL -->
		</Card.Description>
	</Card.Header>

	<Card.Content>
		<div class="grid grid-cols-3 items-center gap-2">
			<!-- ? SERVING SIZE -->
			<Label for="serving{foodNormalized.id}" class="text-right">Serving Size</Label>
			<Input id="serving{foodNormalized.id}" type="number" bind:value={foodNormalized.serving_size} readonly={!editable} class={editable ? "bg-white" : "bg-gray-50"}/>
			<Input id="units{foodNormalized.id}" type="text" bind:value={foodNormalized.unit} readonly={!editable} class={editable ? "bg-white" : "bg-gray-50"}/>
			<!-- ? CALORIES -->
			<Label for="cal{foodNormalized.id}" class="text-right">Calories</Label>
			<Input id="cal{foodNormalized.id}" type="number" bind:value={foodNormalized.normalized_calories} readonly={!editable} class={editable ? "bg-white" : "bg-gray-50"}/>
			<Label class="text-left">kcal</Label>
			<!-- ? PROTEIN -->
			<Label for="pro{foodNormalized.id}" class="text-right">Protein</Label>
			<Input id="pro{foodNormalized.id}" type="number" bind:value={foodNormalized.normalized_protein} class={editable ? "bg-white" : "bg-gray-50"} readonly={!editable}/>
			<Label class="text-left">g</Label>
			<!-- ? CARBS -->
			<Label for="carb{foodNormalized.id}" class="text-right">Carbohydrate</Label>
			<Input id="carb{foodNormalized.id}" type="number" bind:value={foodNormalized.normalized_carbohydrate} readonly={!editable} class={editable ? "bg-white" : "bg-gray-50"} />
			<Label class="text-left">g</Label>
			<!-- ? TOTAL FAT -->
			<Label for="fat{foodNormalized.id}" class="text-right">Total Fat</Label>
			<Input id="fat{foodNormalized.id}" type="number" bind:value={foodNormalized.normalized_fat} readonly={!editable} class={editable ? "bg-white" : "bg-gray-50"}/>
			<Label class="text-left">g</Label>
		</div>
	</Card.Content>
	<Card.Footer>
		<div class="flex w-full justify-between">
		<Button on:click={() => editable = !editable} variant="secondary">
			{#if !editable}
			<PencilLine class="mr-2 h-4 w-4"/>
			Edit
			{:else}
			<PenOff class="mr-2 h-4 w-4"/>
			Cancel
			{/if}
		</Button>

		{#if editable}
			<Button on:click={updateFood}>
				<Check class="mr-2 h-4 w-4" />
				Save
			</Button>
		{/if}

		<Button variant="destructive" on:click={() => onDelete(foodNormalized)}>
			<Trash2 class="mr-2 h-4 w-4"/>
			Delete
		</Button>
		</div>
	</Card.Footer>
</Card.Root>



