<script>
import { invoke } from "@tauri-apps/api";
import { onMount } from "svelte";
import { logId } from './store.js';

let logs = []; 

async function refreshLogs() {
    logs = await invoke('get_all_logs'); 
    logs.sort((a, b) => new Date(b.entry_date) - new Date(a.entry_date));
}

onMount(async () => {
    await refreshLogs(); 
})

</script>

<h3>Daily Log History</h3>
<table>
    <thead>
        <tr>
            <th>Date</th>
            <th>Protein (g)</th>
            <th>Carbohydrates (g)</th>
            <th>Fats (g)</th>
            <th>Calories (kcal)</th>
            <th>Weight (kg)</th>
        </tr>
    </thead>

    <tbody>
        {#each logs as log (log.id)}
            <tr class:highlight={log.id === $logId}>
                <td>{log.entry_date.slice(0,10)}</td>
                <td>{log.total_protein.toFixed(1)}</td>
                <td>{log.total_carbohydrate.toFixed(1)}</td>
                <td>{log.total_fat.toFixed(1)}</td>
                <td>{log.total_calories.toFixed(1)}</td>
                <td>{log.weight.toFixed(2)}</td>
            </tr>
        {/each}
</table>

<style>
    .highlight {
        background-color: yellow 
    }
</style>