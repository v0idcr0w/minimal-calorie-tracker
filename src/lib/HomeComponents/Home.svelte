<script>
	import { invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';
	import { logId, userGoal } from '$lib/store.js';
	import { _ } from 'svelte-i18n'; 
	import MacrosPie from './MacrosPie.svelte';

	// Components
	import * as Card from '$lib/components/ui/card'; 
	import * as Select from '$lib/components/ui/select'; 
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Button } from '$lib/components/ui/button';

	// Svg 
	import { PencilLine } from 'lucide-svelte';
	import { PenOff } from 'lucide-svelte'; 
	import { Check } from 'lucide-svelte'; 

	// this needs to be declared as reactive in order to make the chart reactive too
	$: totalMacros = newUserGoal.protein + newUserGoal.carbohydrate + newUserGoal.fat;
	$: calories = newUserGoal.protein * 4.0 + newUserGoal.carbohydrate * 4.0 + newUserGoal.fat * 9.0; 
	$: macros = [
		(newUserGoal.protein / totalMacros) * 100,
		(newUserGoal.carbohydrate / totalMacros) * 100,
		(newUserGoal.fat / totalMacros) * 100
	];

	// NEW CODE

	let newWeightEditable = false; 
	let newWeight = null; 
	let newUnits = {value: 'Kg', label: 'kg', disabled: false}; // Default 
	let displayedUnits = ''; 
	let goalsEditable = false; 
	let todaysLog = {};
	let newUserGoal = {};
	
	async function getOrCreateTodaysLog() {
		todaysLog = await invoke('get_todays_log');
		logId.set(todaysLog.id);
	}

	async function getOrCreateUserGoal() {
		// if the user is launching the app for the first time, this will create the user goal
		userGoal.set(await invoke('get_user_goal'));
		newUserGoal = { ...$userGoal }; // copy the object
	}

	onMount(async () => {
		await getOrCreateTodaysLog();
		await getOrCreateUserGoal();
		if (!todaysLog.weight) {
			newWeightEditable = true; 
		} else {
			newWeight = todaysLog.weight;
			displayedUnits = todaysLog.units.toLowerCase(); 
		}
	});

	async function updateWeight() {
		try {
			const updatedLog = await invoke('weight_in', { logId: $logId, weight: Number(newWeight), units: newUnits.value});
			todaysLog = {...updatedLog};
			newWeightEditable = false; 
			displayedUnits = newUnits.value;
		} catch(error) {
			console.log(error); 
		}
	}

	async function updateUserGoal() {
		try {
			newUserGoal = await invoke('update_user_goal', { newUserGoal: 
				{...newUserGoal, 
				protein: Number(newUserGoal.protein), 
				carbohydrate: Number(newUserGoal.carbohydrate), fat: Number(newUserGoal.fat), weight: Number(newUserGoal.weight), calories: Number(newUserGoal.calories)
				}
			});
			userGoal.set(newUserGoal);
			goalsEditable = false; 
		} catch(error) {
			console.log(error); 
		}
	}

	function cancelUpdateWeight() {
		displayedUnits = todaysLog.units.toLowerCase(); 
		newWeight = todaysLog.weight;
		newWeightEditable = !newWeightEditable;
		newUnits = null; 
	}

	function cancelUpdateGoal() {
		newUserGoal = { ...$userGoal }; // re-copy 
		goalsEditable = !goalsEditable;
	}


</script>

<div class="mx-4 space-y-4">
	<Card.Root>
		<Card.Header>
			<Card.Title>Weight-in</Card.Title>
			<Card.Description>Insert or edit your weight for today in whatever units you prefer.</Card.Description>
		</Card.Header>
		<Card.Content>
			<div class="grid grid-cols-3 gap-4 w-1/2 mx-auto">
				<Input type="number" class="col-span-2 {newWeightEditable ? 'bg-background' : 'bg-muted/60'}" bind:value={newWeight} readonly={!newWeightEditable} />
				<Select.Root bind:selected={newUnits} disabled={!newWeightEditable}>
					<Select.Trigger class="{newWeightEditable ? 'bg-background' : 'bg-muted/60'} " >
						<Select.Value placeholder={displayedUnits??'Units'}/>
					</Select.Trigger>
					<Select.Content >
						<Select.Item value="Kg" label="kg"/>
						<Select.Item value="Lbs" label="lbs"/>
					</Select.Content>
				</Select.Root>
			</div>
		</Card.Content>
		<Card.Footer class="flex justify-between">
			<Button variant="secondary" on:click={() => cancelUpdateWeight()} disabled={!todaysLog.weight}>
				{#if newWeightEditable}
				<PenOff class="mr-2 w-4 h-4" /> Cancel 
				{:else}
				<PencilLine class="mr-2 w-4 h-4"/> Edit
				{/if}
			</Button>
			{#if newWeightEditable}
			<Button disabled={!newUnits || !newWeight || newWeight < 0} on:click={() => updateWeight()}>
				<Check class="mr-2 w-4 h-4" />
				Save
			</Button>
			{/if}
		</Card.Footer>
	</Card.Root>

	<!-- ************* -->
	<!-- GOALS SECTION -->
	<!-- ************* -->

	<Card.Root>
		<Card.Header>
			<Card.Title>Goals</Card.Title>
			<Card.Description>Set up your weight, calories and macronutrient targets.</Card.Description>
		</Card.Header>
		<Card.Content>
			<div class="grid grid-cols-6 gap-4">
				<Label class="text-right">Weight</Label>
				<Input type="number" bind:value={newUserGoal.weight} readonly={!goalsEditable} class="{goalsEditable ? 'bg-background' : 'bg-muted/60'}"/>

				<Label class="text-right">Daily Calories<br/>(kcal)</Label>
				<Input type="number" readonly={!goalsEditable} bind:value={newUserGoal.calories} class="col-span-3 {goalsEditable ? 'bg-background' : 'bg-muted/60'}"/>

				<Label class="text-right">Protein<br/>(g)</Label>
				<Input type="number" readonly={!goalsEditable} bind:value={newUserGoal.protein} class="{goalsEditable ? 'bg-background' : 'bg-muted/60'}"/>
				
				<Label class="text-right">Carbohydrate<br/>(g)</Label>
				<Input type="number" class="{goalsEditable ? 'bg-background' : 'bg-muted/60'}" readonly={!goalsEditable} bind:value={newUserGoal.carbohydrate}/>
				
				<Label class="text-right">Total Fat<br/>(g)</Label>
				<Input type="number" class="{goalsEditable ? 'bg-background' : 'bg-muted/60'}" readonly={!goalsEditable} bind:value={newUserGoal.fat}/>
			</div>
			<div class="flex justify-between mt-4">
				<Button variant="secondary" on:click={() => cancelUpdateGoal()}>
					{#if goalsEditable}
					<PenOff class="mr-2 w-4 h-4" /> Cancel 
					{:else}
					<PencilLine class="mr-2 w-4 h-4"/> Edit
					{/if}
				</Button>
				{#if goalsEditable}
				<Button on:click={() => updateUserGoal()}
				disabled={!newUserGoal.weight || !newUserGoal.calories || !newUserGoal.protein || !newUserGoal.carbohydrate || !newUserGoal.fat || isNaN(newUserGoal.weight) || isNaN(newUserGoal.calories) || isNaN(newUserGoal.carbohydrate) || isNaN(newUserGoal.protein) || isNaN(newUserGoal.fat) || newUserGoal.weight < 0 || newUserGoal.calories < 0 || newUserGoal.protein < 0 || newUserGoal.carbohydrate < 0 || newUserGoal.fat < 0}	
				>
					<Check class="mr-2 w-4 h-4" />
					Save
				</Button>
				{/if}
			</div>
		</Card.Content>

		<Card.Footer>
			<div class="flex flex-col items-center mx-auto">
				<div>
					<Label>{$_('home.estimateCals')}</Label>
					<span class="text-sm">{calories} kcal</span>
				</div>
				<div class="-my-10">
					<MacrosPie {macros} />
				</div>
			</div>
		</Card.Footer>
	</Card.Root>
</div>
