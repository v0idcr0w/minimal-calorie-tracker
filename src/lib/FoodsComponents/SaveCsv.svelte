<script>
	import { save, message } from '@tauri-apps/api/dialog';
	import { invoke } from '@tauri-apps/api/tauri';
	import { _ } from 'svelte-i18n'; 
	import { Button } from '$lib/components/ui/button'; 
	import { Download } from 'lucide-svelte'; 

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

<Button on:click={saveFile} variant="secondary" class="uppercase text-xs">
	<Download class="mr-2 h-4 w-4"/>
	{$_('foods.save')}
</Button>
