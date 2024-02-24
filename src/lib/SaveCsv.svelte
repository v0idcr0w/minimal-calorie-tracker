<script>
	import { save, message } from '@tauri-apps/api/dialog';
	import { invoke } from '@tauri-apps/api/tauri';
	import { _ } from 'svelte-i18n'; 

	async function saveFile() {
		const filePath = await save({
			filters: [
				{
					name: 'CSV',
					extensions: ['csv']
				}
			]
		});

		if (filePath) {
			await invoke('write_foods_file', { filePath });
			await message('File saved successfully', 'Message');
		}
	}
</script>

<button
	on:click={saveFile}
	class="w-30 flex justify-center items-center rounded bg-primary px-6 pb-2 pt-2.5 text-xs uppercase tracking-tighter font-medium leading-normal text-neutral-600 shadow-[0_4px_9px_-4px_#3b71ca] transition duration-150 ease-in-out hover:bg-primary-600 hover:shadow-[0_8px_9px_-4px_rgba(59,113,202,0.3),0_4px_18px_0_rgba(59,113,202,0.2)] focus:bg-primary-600 focus:shadow-[0_8px_9px_-4px_rgba(59,113,202,0.3),0_4px_18px_0_rgba(59,113,202,0.2)] focus:outline-none focus:ring-0 active:bg-primary-700 active:shadow-[0_8px_9px_-4px_rgba(59,113,202,0.3),0_4px_18px_0_rgba(59,113,202,0.2)]"
>
	<svg
		xmlns="http://www.w3.org/2000/svg"
		height="24px"
		viewBox="0 0 24 24"
		width="24px"
		fill="currentColor"
		><path d="M0 0h24v24H0z" fill="none" /><path
			d="M19 9h-4V3H9v6H5l7 7 7-7zM5 18v2h14v-2H5z"
		/></svg
	>
	{$_('foods.save')}
</button>
