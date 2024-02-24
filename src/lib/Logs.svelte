<script>
	import { invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';
	import { logId, userGoal } from './store.js';
	import { _ } from 'svelte-i18n'; 

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
		if (!$userGoal) {
			userGoal.set(await invoke('get_user_goal'));
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
		</tr>
	</thead>

	<tbody>
		{#each logs as log (log.id)}
			<tr class="border-b transition duration-300 ease-in-out tracking-tight hover:bg-neutral-200">
				<td>{log.entry_date.slice(0, 10)}</td>
				<td
					>{log.total_protein.toFixed(0)}
					<span class="smaller">({(log.total_protein - $userGoal.protein).toFixed(0)}) </span>
				</td>
				<td
					>{log.total_carbohydrate.toFixed(0)}
					<span class="smaller"
						>({(log.total_carbohydrate - $userGoal.carbohydrate).toFixed(0)})</span
					>
				</td>
				<td
					>{log.total_fat.toFixed(0)}
					<span class="smaller">({(log.total_fat - $userGoal.fat).toFixed(0)})</span>
				</td>
				<td
					>{log.total_calories.toFixed(0)}
					<span class="smaller">({(log.total_calories - $userGoal.calories).toFixed(0)})</span>
				</td>
				<td>{log.weight.toFixed(2)} </td>
			</tr>
		{/each}
	</tbody>
</table>

<style lang="postcss">
	.smaller {
		font-size: 0.8em;
	}

	table {
		text-align: center;
		table-layout: fixed;
		width: 90%;
	}

	td,
	th {
		width: 16.66%;
		padding-top: 0.5em;
		padding-bottom: 0.5em;
	}

	thead {
		text-align: center;
	}
</style>
