<script>
	import { invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';
	import { logId } from '../store.js';
	import { _ } from 'svelte-i18n'; 
	import SingleLog from './SingleLog.svelte';

	//components
	import * as Table from '$lib/components/ui/table';

	// state 
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

<div class="grid mx-4">

<Table.Root>
	<Table.Header>
		<Table.Row>
			<Table.Head >{$_('date')}</Table.Head>
			<Table.Head >{$_('protein')}<br/>(g)</Table.Head>
			<Table.Head>Carbohydrate<br/>(g)</Table.Head>
			<Table.Head>Total Fat<br/>(g)</Table.Head>
			<Table.Head>{$_('calories')}<br/>(kcal)</Table.Head>
			<Table.Head >{$_('weight')}</Table.Head>
			<Table.Head>Options</Table.Head>
		</Table.Row>
	</Table.Header>

	<Table.Body>
		{#each logs as log (log.id)}
			<SingleLog {log} {refreshLogs} />
		{/each}
	</Table.Body>
</Table.Root>
</div>


<!-- <table class="table-auto mx-4">
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
</table> -->

<!-- <style lang="postcss">
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
</style> -->
