<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { toTitleCase } from './titleCase.js';
	import { _ } from 'svelte-i18n';
	import SvgTrash from './SvgTrash.svelte';
	import SvgOk from './SvgOk.svelte';
	import SvgEdit from './SvgEdit.svelte';
	import SvgCancel from './SvgCancel.svelte';
	import EditableField from './EditableField.svelte';

	// props
	export let foodNormalized;
	export let onDelete;

	let updatedFood = {};
	let editableFields = false;

	function enableEditing(foodNormalized) {
		editableFields = !editableFields;
		updatedFood = { ...foodNormalized };
	}

	async function updateFood(food) {
		foodNormalized = await invoke('update_food_normalized', { food });
		editableFields = false;
	}

	async function updateFoodName(food, newName) {
		await invoke('update_food_normalized_name', { food, newName });
	}
</script>

<div
	class="block w-full text-center tracking-tighter rounded-lg bg-white p-2 shadow-[0_2px_15px_-3px_rgba(0,0,0,0.07),0_10px_20px_-2px_rgba(0,0,0,0.04)]"
>
	<!-- new rename-->
	<EditableField text={toTitleCase(foodNormalized.name)} handleRename={updateFoodName} obj={foodNormalized} />

	<!-- Button that controls editing the rest of the fields -->
	<p>
		<button class="icon-button" on:click={() => enableEditing(foodNormalized)}>
			{#if !editableFields}
				<SvgEdit />
			{:else}
				<SvgCancel />
			{/if}
		</button>
		<button class="icon-button mb-2 mx-2" on:click={() => onDelete(foodNormalized)}>
			<SvgTrash />
		</button>
	</p>

	<!-- TABLES BELOW -->

	{#if !editableFields}
		<table class="mx-auto">
			<tr>
				<td class="whitespace-nowrap px-2 py-0.5 font-bold">{$_('servingSize')}</td>
				<td>{foodNormalized.serving_size}</td>
				<td class="text-right">{foodNormalized.unit}</td>
			</tr>
			<tr>
				<td class="whitespace-nowrap px-2 py-0.5 font-bold">{$_('calories')}</td>
				<td>{foodNormalized.normalized_calories}</td>
				<td class="text-right">kcal</td>
			</tr>
			<tr>
				<td class="whitespace-nowrap px-2 py-0.5 font-bold">{$_('protein')}</td>
				<td>{foodNormalized.normalized_protein}</td>
				<td class="text-right">g</td>
			</tr>
			<tr>
				<td class="whitespace-nowrap px-2 py-0.5 font-bold">{$_('carbohydrates')}</td>
				<td>{foodNormalized.normalized_carbohydrate}</td>
				<td class="text-right">g</td>
			</tr>
			<tr>
				<td class="whitespace-nowrap px-2 py-0.5 font-bold">{$_('fats')}</td>
				<td>{foodNormalized.normalized_fat}</td>
				<td class="text-right">g</td>
			</tr>
		</table>
	{:else}
		<table class="mx-auto">
			<tr>
				<td class="whitespace-nowrap px-2 py-0.5 font-bold">{$_('servingSize')}</td>
				<td
					><input
						class="w-full"
						name="serving_size"
						type="number"
						min="0"
						bind:value={updatedFood.serving_size}
					/></td
				>
				<td class="text-right"
					><input class="w-full" name="unit" bind:value={updatedFood.unit} placeholder="Unit" /></td
				>
			</tr>
			<tr>
				<td class="whitespace-nowrap px-2 py-0.5 font-bold">{$_('calories')}</td>
				<td
					><input
						class="w-full"
						name="calories_size"
						type="number"
						min="0"
						bind:value={updatedFood.normalized_calories}
					/></td
				>
				<td>kcal</td>
			</tr>
			<tr>
				<td class="whitespace-nowrap px-2 py-0.5 font-bold">{$_('protein')}</td>
				<td>
					<input
						class="w-full"
						name="protein"
						type="number"
						min="0"
						bind:value={updatedFood.normalized_protein}
					/>
				</td>
				<td>g</td>
			</tr>
			<tr>
				<td class="whitespace-nowrap px-2 py-0.5 font-bold">{$_('carbohydrates')}</td>
				<td
					><input
						class="w-full"
						name="carbohydrate"
						type="number"
						min="0"
						bind:value={updatedFood.normalized_carbohydrate}
					/></td
				>
				<td>g</td>
			</tr>
			<tr>
				<td class="whitespace-nowrap px-2 py-0.5 font-bold">{$_('fats')}</td>
				<td
					><input
						class="w-full"
						name="fat"
						type="number"
						min="0"
						bind:value={updatedFood.normalized_fat}
					/></td
				>
				<td>g</td>
			</tr>
		</table>
		<button class="icon-button" on:click={() => updateFood(updatedFood)}>
			<SvgOk />
		</button>
	{/if}
</div>
