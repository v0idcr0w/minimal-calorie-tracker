<script>
	import { invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';
	import { logId } from '../store.js';
	import { _ } from 'svelte-i18n'; 
	import SingleLog from './SingleLog.svelte';

	let logs = [];

	async function refreshLogs() {
		logs = await invoke('get_all_logs');
		logs.sort((a, b) => new Date(b.entry_date) - new Date(a.entry_date));
	}

	onMount(async () => {
		await refreshLogs();
		if (!$logId) {
			logId.set(await invoke('get_todays_log').id);
		}
	});
</script>

<table class="table-auto mx-4">
	<thead class="border-b bg-purple-800 font-medium text-white tracking-tight">
		<tr>
			<th>{$_('date')} <br /> </th>
			<th>{$_('protein')} <br /> (g)</th>
			<th>{$_('carbohydrates')} <br /> (g)</th>
			<th>{$_('fats')} <br /> (g)</th>
			<th>{$_('calories')} <br /> (kcal)</th>
			<th>{$_('weight')} <br /> (kg)</th>
			<th>{$_('logs.edit')} <br /> </th>
			<th>{$_('logs.delete')}</th>
		</tr>
	</thead>

	<tbody>
		{#each logs as log (log.id)}
			<SingleLog {log} {refreshLogs} />
		{/each}
	</tbody>
</table>

<style lang="postcss">
	table {
		text-align: center;
		table-layout: fixed;
		width: 90%;
	}

	th {
		width: 16.66%;
		padding-top: 0.5em;
		padding-bottom: 0.5em;
	}

	thead {
		text-align: center;
	}
</style>
