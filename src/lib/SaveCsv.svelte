<script>
    import { save, message } from '@tauri-apps/api/dialog'; 
    import { invoke } from '@tauri-apps/api/tauri'; 

    async function saveFile() {
        const filePath = await save({
        filters: [{
            name: 'CSV',
            extensions: ['csv']
            }]
        });

        if (filePath) {
            await invoke('write_foods_file', { filePath });
            await message('File saved successfully', 'Message');
        }
    }
</script>

<button on:click={saveFile}>Save as CSV</button>