<script>
    export let log; 
    export let refreshLogs; 
    import { invoke } from '@tauri-apps/api';
    import { confirm } from '@tauri-apps/api/dialog';
    import { logId } from './store.js';
    import { _ } from 'svelte-i18n';
    import { formatDate } from './formatDate.js';
    import GradientButton from './GradientButton.svelte'; 
    import SvgEdit from './SvgEdit.svelte';
    import SvgOk from './SvgOk.svelte';
    import SvgRemove from './SvgRemove.svelte';


    let editable = false; 
    let protein, carbohydrate, fat, calories, weight;
    const logDate = new Date(log.entry_date); 

    function autofocus(node) {
        node.focus(); 
    }

    function sanitize(input) {
        // checks if parseFloat was successful. 
        if (isNaN(input)) {
            return 0; 
        }
        return input; 
    }

    async function updateLog() {
        const editedLog = {
            ...log, 
            total_protein: sanitize(parseFloat(protein.textContent)),
            total_carbohydrate: sanitize(parseFloat(carbohydrate.textContent)),
            total_fat: sanitize(parseFloat(fat.textContent)),
            total_calories: sanitize(parseFloat(calories.textContent)),
            weight: sanitize(parseFloat(weight.textContent))
        }
        log = await invoke('update_log_standalone', { log: editedLog });
        editable = false; 
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

<tr class="border-b transition duration-300 ease-in-out tracking-tight hover:bg-neutral-200">
    <td>{formatDate(logDate)}</td>
    {#if !editable}
        <td
            >{log.total_protein.toFixed(0)}
        </td>
        <td
            >{log.total_carbohydrate.toFixed(0)}
        </td>
        <td
            >{log.total_fat.toFixed(0)}
        </td>
        <td
            >{log.total_calories.toFixed(0)}
        </td>
        <td>{log.weight.toFixed(2)} </td>
        <!-- edit button is disabled if the current log id is equal to whatever is in store  -->
        <td> <GradientButton onClick={() => editable = true} disabled={log.id === $logId} > <SvgEdit />  </GradientButton> </td>
    {:else}
        <!-- autofocus on this to let the user know that it is editable -->
        <td bind:this={protein} contenteditable="true" use:autofocus
            >{log.total_protein.toFixed(0)}
        </td>
        <td bind:this={carbohydrate} contenteditable="true"
            >{log.total_carbohydrate.toFixed(0)}
        </td>
        <td bind:this={fat} contenteditable="true"
            >{log.total_fat.toFixed(0)}
        </td>
        <td bind:this={calories} contenteditable="true"
            >{log.total_calories.toFixed(0)}
        </td>
        <td bind:this={weight} contenteditable="true">{log.weight.toFixed(2)} </td>
        <td> <GradientButton onClick={updateLog} disabled={log.id === $logId} > <SvgOk />  </GradientButton> </td>
    {/if}
        <td><GradientButton onClick={deleteLog} disabled={log.id === $logId}> <SvgRemove /> </GradientButton> </td>
</tr>


<style lang="postcss">
	td {
		width: 16.66%;
		padding-top: 0.5em;
		padding-bottom: 0.5em;
	}

</style>