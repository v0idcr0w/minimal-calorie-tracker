<script>
    import { invoke } from '@tauri-apps/api'; 
    import { onMount } from 'svelte'; 
    import { _ } from 'svelte-i18n';
    import { formatDate } from '../formatDate';

    // Components
	import * as Card from '$lib/components/ui/card';
    import * as Table from "$lib/components/ui/table";
	import { Label } from '$lib/components/ui/label';
	import { Button } from '$lib/components/ui/button';
    import { Switch } from '$lib/components/ui/switch';
    import * as AlertDialog from "$lib/components/ui/alert-dialog";
    import Collapse from '$lib/Collapse.svelte';

    // Icons
    import { X } from 'lucide-svelte';

    // props
    export let meal; 
    export let refreshMeals; 

    // state
    let foods = [];
    let alertDialogOpen = false; 
    let mealMacros = {protein: 0, carbohydrate: 0, fat: 0, calories: 0}; 
    const date = new Date(meal.entry_timestamp); 

    async function getFoods() {
        foods = await invoke('get_foods_by_meal_id', { mealId: meal.id }); 
        mealMacros = await invoke('compute_meal_macros', { mealId: meal.id });
    }

    async function deleteMeal() {
        await invoke('delete_meal', { meal }); 
        await refreshMeals(); 
    }

    async function handleToggle(checked) {
        try {
            await invoke('update_meal_is_constant', { mealId: meal.id, status: checked });
            meal.is_constant = checked; 
        } catch(error) {
            console.error(error); 
        }
    }


    onMount(async () => {
        await getFoods(); 
    });



</script>


<Card.Root>
    <div class="flex w-full justify-right">
        <Button class="ml-auto" variant="ghost" size="icon" on:click={() => alertDialogOpen = true}>
            <X class="w-4 h-4" /> 
        </Button>
        <AlertDialog.Root bind:open={alertDialogOpen}>
            <AlertDialog.Trigger />
            <AlertDialog.Content>
                <AlertDialog.Header>
                    <AlertDialog.Title>Are you absolutely sure?</AlertDialog.Title>
                    <AlertDialog.Description>
                      This action cannot be undone. Please confirm your choice. 
                    </AlertDialog.Description>
                  </AlertDialog.Header>
                  <AlertDialog.Footer>
                    <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
                    <AlertDialog.Action on:click={() => deleteMeal()}>Confirm</AlertDialog.Action>
                  </AlertDialog.Footer>
            </AlertDialog.Content>
        </AlertDialog.Root>
    </div>
    <Card.Header class="pt-0 pb-2">
        <Card.Title class="capitalize">
            {meal.name}
        </Card.Title>
        <Card.Description class="pb-2">
            <div class="flex items-center justify-between">
            {$_('meal_history.created')} {formatDate(date)} 
                <div class="flex items-center space-x-2">
                    <Switch includeInput={true} id="is-constant" checked={meal.is_constant} onCheckedChange={handleToggle}/>
                    <Label for="is-constant">Constant</Label>
                </div>
            </div>
        </Card.Description>
        <div class="grid space-y-2">
            <div class="grid grid-cols-1 gap-2 text-sm">
                {#each foods as food (food.id)}
                    <li>
                        <span class='capitalize'>{food.name}</span> {food.amount} {food.unit}
                    </li>
                {/each}
            </div>
            {#if Object.keys(mealMacros).length !== 0}
            <Collapse title={"Show meal calories and macros"}>
                        <Table.Root>
                            <Table.Row>
                                <Table.Head>Calories</Table.Head>
                                <Table.Cell>{mealMacros.calories.toFixed(0)}</Table.Cell>
                                <Table.Cell>kcal</Table.Cell>
                            </Table.Row>
                            <Table.Row>
                                <Table.Head>Protein</Table.Head>
                                <Table.Cell>{mealMacros.protein.toFixed(1)}</Table.Cell>
                                <Table.Cell>g</Table.Cell>
                            </Table.Row>
                            <Table.Row>
                                <Table.Head>Carbohydrate</Table.Head>
                                <Table.Cell>{mealMacros.carbohydrate.toFixed(1)}</Table.Cell>
                                <Table.Cell>g</Table.Cell>
                            </Table.Row>
                            <Table.Row>
                                <Table.Head>Total Fat</Table.Head>
                                <Table.Cell>{mealMacros.fat.toFixed(1)}</Table.Cell>
                                <Table.Cell>g</Table.Cell>
                            </Table.Row>
                        </Table.Root>
                    </Collapse>
            {/if}
        </div>
    </Card.Header>
</Card.Root>