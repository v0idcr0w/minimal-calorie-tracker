<script>
	import { invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';
	import { logId, userGoal } from './store.js';
	import { _ } from 'svelte-i18n'; 
	import MacrosPie from './MacrosPie.svelte';
	import MaterialFloatingLabel from './MaterialFloatingLabel.svelte';
	import MaterialFloatingLabelError from './MaterialFloatingLabelError.svelte';
	import GradientButton from './GradientButton.svelte';
	import Tooltip from './Tooltip.svelte';
	import SvgOk from './SvgOk.svelte';
	import SvgCancel from './SvgCancel.svelte';
	import SvgEdit from './SvgEdit.svelte';

	let weight = 0;
	// button controls
	let editWeight = false;
	let editWeightGoal = false;
	let editCaloriesGoal = false;
	let editMacrosGoal = false;

	let todaysLog = {};

	let newUserGoal = {};
	// this needs to be declared as reactive in order to make the chart reactive too
	$: totalMacros = newUserGoal.protein + newUserGoal.carbohydrate + newUserGoal.fat;
	$: calories = newUserGoal.protein * 4.0 + newUserGoal.carbohydrate * 4.0 + newUserGoal.fat * 9.0; 
	$: macros = [
		(newUserGoal.protein / totalMacros) * 100,
		(newUserGoal.carbohydrate / totalMacros) * 100,
		(newUserGoal.fat / totalMacros) * 100
	];

	async function getOrCreateTodaysLog() {
		todaysLog = await invoke('get_todays_log');
		weight = todaysLog.weight;
		logId.set(todaysLog.id);
	}

	async function getOrCreateUserGoal() {
		// if the user is launching the app for the first time, this will create the user goal
		userGoal.set(await invoke('get_user_goal'));
		newUserGoal = { ...$userGoal }; // copy the object
	}

	async function addWeight(weight) {
		if (validateInput(weight)) {
			await invoke('weight_in', { logId: $logId, weight });
			todaysLog.weight = weight;
			editWeight = false;
		}
	}

	// functions to edit the user's goal
	async function updateWeightGoal() {
		// turn into number (currently string)
		userGoal.set(await invoke('update_weight_goal', { newUserGoal }));
		editWeightGoal = false;
	}

	async function updateCaloriesGoal() {
		userGoal.set(await invoke('update_calories_goal', { newUserGoal }));
		editCaloriesGoal = false;
	}

	async function updateMacrosGoal() {
		userGoal.set(await invoke('update_macros_goal', { newUserGoal }));
		editMacrosGoal = false;
	}

	onMount(async () => {
		await getOrCreateTodaysLog();
		await getOrCreateUserGoal();
	});

	let validationError = '';
	function validateInput(weight) {
		if (weight === undefined) {
			validationError = $_('error.undefinedWeight');
			return false;
		}
		if (weight < 0 || weight > 999) {
			validationError = $_('error.invalidWeight');
			return false;
		}
		return true;
	}
</script>

<div class="mx-4">
	<div class="flex items-center justify-center">
		<h3 class="text-neutral-700 text-xl m-2 font-bold">{$_('home.weightIn')}:</h3>
		
		{#if todaysLog.weight > 0}
			<h3 class="text-neutral-700 text-xl my-2 font-bold mx-2">{todaysLog.weight} kg</h3>

			<button class="icon-button" on:click={() => (editWeight = !editWeight)}>
				{#if !editWeight}
					<SvgEdit />
				{:else}
					<SvgCancel />
				{/if}
			</button>
		{/if}
	</div>

	<!-- EDITING WEIGHT -->
	<div class="flex flex-col items-center justify-center">
		{#if (todaysLog.weight <= 0 || editWeight) && !validationError}
			<MaterialFloatingLabel bind:value={weight} label="{$_('home.newWeight')} (kg)" type="number" />
		{/if}

		{#if validationError && (editWeight || todaysLog.weight <= 0) } 
			<MaterialFloatingLabelError
				bind:value={weight}
				error={$_('home.invalidWeight')}
				errorMessage={validationError}
				type="number"
			/>
		{/if}

		{#if editWeight || todaysLog.weight <= 0}
			<button class="icon-button" on:click={addWeight(weight)}>
				<SvgOk />
			</button>
		{/if}
	</div>

	<!-- ************* -->
	<!-- GOALS SECTION -->
	<!-- ************* -->

	<div
		class="flex flex-col items-center justify-center border-b-2 border-neutral-200 border-opacity-100"
	>
		<h3 class="text-neutral-700 text-xl m-4 font-bold">{$_('home.goals')}</h3>

		<ul
			class="w-72 text-lg tracking-tight font-medium text-neutral-900 bg-white border border-gray-200 rounded-lg"
		>
			<li class="w-full px-4 py-2 border-b border-gray-200 rounded-t-lg">
				{$_('weight')}

				{#if !editWeightGoal}
					<GradientButton onClick={() => (editWeightGoal = !editWeightGoal)}>
						<SvgEdit />
					</GradientButton>
				{:else}
					<GradientButton onClick={() => (editWeightGoal = !editWeightGoal)}>
						<SvgCancel />
					</GradientButton>
				{/if}

				<!--CONFIRMATION BUTTON  -->
				{#if editWeightGoal}
					<GradientButton onClick={updateWeightGoal}>
						<SvgOk />
					</GradientButton>
				{/if}

				{#if editWeightGoal}
					<div class="mt-4">
						<MaterialFloatingLabel
							bind:value={newUserGoal.weight}
							label={$_('home.newTarget')}
							type="number"
						/>
					</div>
				{:else}
					<p class="text-sm tracking-tight">
						{$userGoal.weight} kg
					</p>
				{/if}
			</li>

			<li class="w-full px-4 py-2 border-b border-gray-200">
				{$_('home.dailyCalories')}
				{#if !editCaloriesGoal}
					<GradientButton onClick={() => (editCaloriesGoal = !editCaloriesGoal)}>
						<SvgEdit />
					</GradientButton>
				{:else}
					<GradientButton onClick={() => (editCaloriesGoal = !editCaloriesGoal)}>
						<SvgCancel />
					</GradientButton>
				{/if}

				{#if editCaloriesGoal}
					<GradientButton onClick={updateCaloriesGoal}>
						<SvgOk />
					</GradientButton>
				{/if}

				{#if editCaloriesGoal}
					<div class="mt-4">
						<MaterialFloatingLabel
							bind:value={newUserGoal.calories}
							label={$_('home.newTarget')}
							type="number"
						/>
					</div>
				{:else}
					<p class="text-sm tracking-tight">
						{$userGoal.calories} kcal
					</p>
				{/if}
			</li>
			<li class="w-full px-4 py-2">
				{$_('home.macronutrients')}
				
				{#if !editMacrosGoal}
					<GradientButton onClick={() => (editMacrosGoal = !editMacrosGoal)}>
						<SvgEdit />
					</GradientButton>
				{:else}
					<GradientButton onClick={() => (editMacrosGoal = !editMacrosGoal)}>
						<SvgCancel />
					</GradientButton>
				{/if}
				{#if editMacrosGoal}
				<GradientButton onClick={updateMacrosGoal}>
					<SvgOk />
				</GradientButton>
				{/if}
				

				{#if editMacrosGoal}
					<div class="flex items-center text-sm mt-4">
						<MaterialFloatingLabel
							type="number"
							label="{$_('protein')} (g)"
							bind:value={newUserGoal.protein}
						/> ≈ {newUserGoal.protein * 4} kcal
					</div>
					<div class="flex items-center text-sm">
						<MaterialFloatingLabel
							type="number"
							label="{$_('carbohydrates')} (g)"
							bind:value={newUserGoal.carbohydrate}
						/> ≈ {newUserGoal.carbohydrate * 4} kcal
					</div>

					<div class="flex items-center text-sm">
						<MaterialFloatingLabel type="number" label="{$_('fats')} (g)" bind:value={newUserGoal.fat} />
						≈ {newUserGoal.fat * 9} kcal
					</div>
				{:else}
					<p class="text-sm tracking-tight my-2">{$userGoal.protein} g {$_('protein')}</p>
					<p class="text-sm tracking-tight my-2">{$userGoal.carbohydrate} g {$_('carbohydrates')}</p>
					<p class="text-sm tracking-tight my-2">{$userGoal.fat} g {$_('fats')}</p>
				{/if}
				<p class="text-sm tracking-tight my-2"><b>{$_('home.estimateCals')}</b> {calories} kcal</p>
				<div class="-mt-10">
				<MacrosPie {macros} />
				</div>
			</li>
		</ul>
	</div>
</div>
