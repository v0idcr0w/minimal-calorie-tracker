<script>
    // This displays all foods stored in their normalized format in the database. 
    import { invoke } from '@tauri-apps/api/tauri'
    import { confirm } from '@tauri-apps/api/dialog';
    import { onMount } from 'svelte'; 
    import { foodsNormalized } from './store.js';
    import SingleFood from './SingleFood.svelte'; 
    
    // components 
    import NewFood from './NewFood.svelte'

    async function refreshFoods() {
      foodsNormalized.set(await invoke('get_foods_normalized'))
    }

    onMount(async () => {
      await refreshFoods() 
    });

    async function deleteFood(food) {
      const confirmed = await confirm('This action cannot be reverted. Are you sure?', { title: 'Confirm', type: 'info' });
      if (!confirmed) return; 
      await invoke('delete_food_normalized', { food })
      // refresh list 
      await refreshFoods()
    }

</script>
  
<NewFood />

{#each $foodsNormalized as foodNormalized (foodNormalized.id)}
<SingleFood foodNormalized={foodNormalized} onDelete={deleteFood} />
{/each}

