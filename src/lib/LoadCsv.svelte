<script>
	import { open, message } from '@tauri-apps/api/dialog';
	import { invoke } from '@tauri-apps/api/tauri';
	import { _ } from 'svelte-i18n'; 

	export let onUpdate;

	async function loadFile() {
		const filePath = await open({
			multiple: false,
			filters: [
				{
					name: 'CSV',
					extensions: ['csv']
				}
			]
		});
		if (filePath) {
			await invoke('read_foods_file', { filePath });
			await onUpdate(); // this will refresh the foods normalized list
			await message('File loaded successfully', 'Message');
		}
	}
</script>

<button
	on:click={loadFile}
	class="w-30 flex justify-center items-center rounded bg-primary px-6 pb-2 pt-2.5 text-xs uppercase tracking-tighter font-medium leading-normal text-neutral-600 shadow-[0_4px_9px_-4px_#3b71ca] transition duration-150 ease-in-out hover:bg-primary-600 hover:shadow-[0_8px_9px_-4px_rgba(59,113,202,0.3),0_4px_18px_0_rgba(59,113,202,0.2)] focus:bg-primary-600 focus:shadow-[0_8px_9px_-4px_rgba(59,113,202,0.3),0_4px_18px_0_rgba(59,113,202,0.2)] focus:outline-none focus:ring-0 active:bg-primary-700 active:shadow-[0_8px_9px_-4px_rgba(59,113,202,0.3),0_4px_18px_0_rgba(59,113,202,0.2)]"
>
	<svg
		xmlns="http://www.w3.org/2000/svg"
		height="24px"
		viewBox="0 0 24 24"
		width="24px"
		fill="currentColor"
		><path d="M0 0h24v24H0z" fill="none" /><path d="M9 16h6v-6h4l-7-7-7 7h4zm-4 2h14v2H5z" /></svg
	>
	{$_('foods.load')}
</button>
