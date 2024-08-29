<script>
    import { invoke } from '@tauri-apps/api';
    import { confirm } from '@tauri-apps/api/dialog';
    import { logId } from '../store.js';
    import { _ } from 'svelte-i18n';
    import { formatDate } from '../formatDate.js';
    
    // components
    import * as Table from '$lib/components/ui/table';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
    import * as Dialog from '$lib/components/ui/dialog';
    import { Button } from '$lib/components/ui/button';
    import { Input } from '$lib/components/ui/input';
    import { Label } from '$lib/components/ui/label';
    
    // svg 
    import { PencilLine } from 'lucide-svelte'; 
    import { X } from 'lucide-svelte';
    import { EllipsisVertical } from 'lucide-svelte';
    
    // props     
    export let log; 
    export let refreshLogs; 
    // state 
    let editDialogOpen = false; 
    let protein = log.total_protein;
    let carbohydrate = log.total_carbohydrate;
    let fat = log.total_fat; 
    let calories = log.total_calories; 
    let weight = log.weight;
    const logDate = new Date(log.entry_date); 

    function estimateCalories() {
        calories = (protein??0) * 4 + (carbohydrate??0) * 4 + (fat??0) * 9;
    }

    async function updateLog() {
        try {
            const editedLog = {
                ...log, 
                total_protein: Number(protein),
                total_carbohydrate: Number(carbohydrate),
                total_fat: Number(fat),
                total_calories: Number(calories),
                weight: Number(weight)
            }
        log = await invoke('update_log_standalone', { log: editedLog });
        editDialogOpen = false; 
        } catch(error) {
            console.log(error); 
        }
    }

    function resetEdit() {
        protein = log.total_protein;
        carbohydrate = log.total_carbohydrate;
        fat = log.total_fat; 
        calories = log.total_calories; 
        weight = log.weight;
    }

    async function deleteLog() {
        const confirmed = await confirm($_('foods.deleteDialog.message'), {
			title: $_('foods.deleteDialog.title'),
			type: 'info'
		});
		if (!confirmed) return;
        await invoke('delete_log', { log });   
        await refreshLogs(); 
    }

</script>

<Table.Row>
    <Table.Cell>{formatDate(logDate)}</Table.Cell>
    <Table.Cell>
        {log.total_protein.toFixed(0)}
    </Table.Cell>
    <Table.Cell>
        {log.total_carbohydrate.toFixed(0)}
    </Table.Cell>
    <Table.Cell>
        {log.total_fat.toFixed(0)}
    </Table.Cell>
    <Table.Cell>
        {log.total_calories.toFixed(0)}
    </Table.Cell>
    <Table.Cell>
        {log.weight.toFixed(2)}
    </Table.Cell>
    <Table.Cell>
        <!-- ! DROPDOWN -->
        <DropdownMenu.Root>
		<DropdownMenu.Trigger disabled={log.id === $logId}>
			<Button variant="ghost" class="rounded-full" size="icon" disabled={log.id === $logId}>
				<EllipsisVertical class="h-4 w-4" />
			</Button>
		</DropdownMenu.Trigger>
		<DropdownMenu.Content>
			<DropdownMenu.Group>
				<DropdownMenu.Label>Options</DropdownMenu.Label>
				<DropdownMenu.Separator /> 
				<DropdownMenu.Item on:click={() => editDialogOpen = true}>
					<PencilLine class="w-4 h-4 mr-2"/>
					Edit
				</DropdownMenu.Item>
				<DropdownMenu.Item on:click={() => deleteLog()}>
					<X class="w-4 h-4 mr-2" />
					Delete
				</DropdownMenu.Item>
			</DropdownMenu.Group>
		</DropdownMenu.Content>
	</DropdownMenu.Root>

    <!-- ! EDIT MODAL -->
    <Dialog.Root bind:open={editDialogOpen}>
        <Dialog.Trigger/>
        <Dialog.Content>
            <Dialog.Header>
                <Dialog.Title>Edit Log for {log.entry_date}</Dialog.Title>
                <Dialog.Description>Calories are automatically estimated based on the amount of protein, carbohydrate and fat consumed. Feel free to overwrite the estimated value.</Dialog.Description>
            </Dialog.Header>
            <div class="grid grid-cols-6 gap-4 ">
                <!-- Three cols -->
                <Label class="text-right text-xs">Protein<br/>(g)</Label>
                <Input bind:value={protein} on:change={() => estimateCalories()} />

                <Label class="text-right text-xs">Carbohydrate<br/>(g)</Label>
                <Input bind:value={carbohydrate} on:change={() => estimateCalories()} />

                <Label class="text-right text-xs">Total Fat<br/>(g)</Label>
                <Input bind:value={fat} on:change={() => estimateCalories()}/>
                
                <!-- Separate line -->
                <Label class="text-right  text-xs">Calories<br/>(kcal)</Label>
                <Input class="col-span-2" bind:value={calories}/>

                <Label class="text-right text-xs">Weight</Label>
                <Input class="col-span-2" bind:value={weight}/>
            </div>
        <Dialog.Footer>
            <Dialog.Close>
                <Button variant="secondary" on:click={() => resetEdit()}>Cancel</Button>
            </Dialog.Close>
            <Button on:click={() => updateLog()}>Confirm</Button>
        </Dialog.Footer>
        </Dialog.Content>
    </Dialog.Root>

    </Table.Cell>
</Table.Row>