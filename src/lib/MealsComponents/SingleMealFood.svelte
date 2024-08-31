<script>
	import { _ } from 'svelte-i18n'; 
	import { createEventDispatcher } from 'svelte';

	// Components
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';

	// Svg icons
	import { Check } from 'lucide-svelte'; 
	import { PencilLine } from 'lucide-svelte';
	import { PenOff } from 'lucide-svelte';
	import { X } from 'lucide-svelte'; 
	import { EllipsisVertical } from 'lucide-svelte'; 

    // props
    export let food; 

    // state
	let editable = false; 
	let newAmount = food.amount; 

    const dispatch = createEventDispatcher();

    function deleteFood() {
        dispatch('delete', food);
    }

    function updateFood() {
        dispatch('update', { food, newAmount: Number(newAmount) }); 
        editable = false; 
    }

    function resetFood() {
        newAmount = food.amount; 
        editable = false; 
    }
</script> 


<div class="flex items-center">
	<!-- Three dots dropdown with choices: Delete, Edit -->
	<div class="mr-2">
	<DropdownMenu.Root>
		<DropdownMenu.Trigger>
			<Button variant="ghost" class="rounded-full" size="icon">
				<EllipsisVertical class="h-4 w-4" />
			</Button>
		</DropdownMenu.Trigger>
		<DropdownMenu.Content>
			<DropdownMenu.Group>
				<DropdownMenu.Label>Options</DropdownMenu.Label>
				<DropdownMenu.Separator /> 
				<DropdownMenu.Item on:click={()=>editable=true}>
					<PencilLine class="w-4 h-4 mr-2"/>
					Edit
				</DropdownMenu.Item>
				<DropdownMenu.Item on:click={deleteFood}>
					<X class="w-4 h-4 mr-2" />
					Delete
				</DropdownMenu.Item>
			</DropdownMenu.Group>
		</DropdownMenu.Content>
	</DropdownMenu.Root>
	</div>
	<div class="grid grid-cols-7 items-center w-full">
		<Label class="capitalize col-span-2 text-sm">{food.name}</Label>
		<Input type="number" bind:value={newAmount} readonly={!editable} class="col-span-2 text-xs {editable ? 'bg-background' : 'bg-muted/60'}" />
		<Label class="col-span-1 text-center text-sm">{food.unit}</Label>
		{#if editable}
		<Button size="icon" variant="ghost" class="rounded-full text-emerald-600" on:click={updateFood} disabled={newAmount < 0}>
			<Check class="h-4 w-4"/>
		</Button>
		<Button on:click={resetFood} variant="ghost" class="rounded-full text-gray-700" size="icon">
			<PenOff class="h-4 w-4" />
		</Button>
		{/if}

	</div>

</div>