<script>
    import { open, message } from '@tauri-apps/api/dialog'; 
    import { invoke } from '@tauri-apps/api/tauri';

    export let onUpdate;

    async function loadFile() {
        const filePath = await open({ 
            multiple: false, 
            filters: [{ 
                name: 'CSV', extensions: ['csv'] 
            }] 
        });
        if (filePath) {
            await invoke('read_foods_file', { filePath });
            await onUpdate(); // this will refresh the foods normalized list 
            await message('File loaded successfully', 'Message');
        }
    }

</script>

<button on:click={loadFile}>Load From CSV</button>
