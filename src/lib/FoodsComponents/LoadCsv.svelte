<script>
	import { open, message } from '@tauri-apps/api/dialog';
	import { invoke } from '@tauri-apps/api/tauri';
	import { _ } from 'svelte-i18n'; 
	import { Button } from '$lib/components/ui/button'; 
	import { Upload } from 'lucide-svelte'; 

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

<Button variant="secondary" on:click={loadFile} class="uppercase text-xs">
	<Upload class="mr-2 h-4 w-4"/>
	{$_('foods.load')}
</Button>
