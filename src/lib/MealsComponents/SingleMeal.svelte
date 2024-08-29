<script>
    import {createEventDispatcher} from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
    import { foodsNormalized, recipes } from '../store.js';
    import { onMount } from 'svelte';
    import SingleMealFood from './SingleMealFood.svelte'; 
    // components 
	import * as Dialog from "$lib/components/ui/dialog"; 
	import * as Card from '$lib/components/ui/card';
    import * as Select from '$lib/components/ui/select';
    import * as RadioGroup from "$lib/components/ui/radio-group";
    import * as Table from "$lib/components/ui/table";
    import Collapse from '$lib/Collapse.svelte';
    import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Button } from '$lib/components/ui/button';
    import { Switch } from '$lib/components/ui/switch';
    // Svg icons
	import { PencilLine } from 'lucide-svelte';
	import { Plus } from 'lucide-svelte'; 
    import { X } from 'lucide-svelte'; 

    // props
    export let meal;
    
    // state 
    let isConstant = meal.is_constant; 
    let isActive = !meal.is_disabled;
    let newName = meal.name; 
    let addFoodDialogOpen = false; 
    let renameDialogOpen = false; 
    let selectionOption = "food"; // default option
    let selectedFood = null;
    let selectedRecipe = null;  
    let selectedAmount = null; 
    let mealFoods = []; 
    let mealMacros = {}; 
 

    const dispatch = createEventDispatcher();

    async function getMealFoods() {
        try {
            mealFoods = await invoke('get_foods_by_meal_id', { mealId: meal.id });
        } catch(error) {
            console.log(error); 
        }
    }

    async function getMealMacros() {
        try {
            mealMacros = await invoke('compute_meal_macros', { mealId: meal.id });
        } catch(error) {
            console.log(error);
        }
    }

    onMount(async () => {
        await getMealFoods(); 
        await getMealMacros(); 
    });

    async function updateConstantStatus(checked) {
        try {
            meal = await invoke('update_meal_is_constant', { mealId: meal.id, status: checked });
            dispatch('update', meal);
            dispatch('refresh'); 
        } catch(error) {
            console.log(error); 
        }
    }

    async function updateDisabledStatus(checked) {
        try {
            meal = await invoke('update_meal_is_disabled', { mealId: meal.id, status: !checked}); 
            dispatch('update', meal); 
            dispatch('updateTotals'); 
        } catch(error) {
            console.log(error); 
        }
    }

    async function handleRename() {
        try {
            meal = await invoke('update_meal_name', { mealId: meal.id, newName });
            dispatch('update', meal); 
            renameDialogOpen = false; 
        } catch(error) {
            console.log(error); 
        }
	}

    async function addNewFoodToMeal() {
        try {
            if (selectedFood && selectedAmount) {
                const selectedId = selectedFood.value.id; 
                const newFood = await invoke('add_new_food', { selectedId, amount: Number(selectedAmount), mealId: meal.id });
                mealFoods = [...mealFoods, newFood];
            } else if (selectedRecipe && selectedAmount) {
                const selectedId = selectedRecipe.value.id;
                const newRecipe = await invoke('add_new_food_from_recipe', { selectedId, amount: Number(selectedAmount), mealId: meal.id });
                mealFoods = [...mealFoods, newRecipe]; 
            } else {
                return; 
            }
            // update daily macros and close the dialog
            await getMealFoods()
            await getMealMacros(); 
            dispatch('updateTotals'); //Still not working properly
            addFoodDialogOpen = false;
            selectedFood = null;
            selectedRecipe = null;  
            selectedAmount = null; 
        } catch(error) {
            console.log(error); 
        }
    }

    async function deleteFoodFromMeal(event) {
        const food = event.detail;
        try {
            await invoke('delete_food', { food });
            mealFoods = mealFoods.filter((f) => f.id !== food.id);
            await getMealMacros(); 
            dispatch('updateTotals'); 
        } catch(error) {
            console.log(error); 
        }
    }
    
    async function editFoodInMeal(event) {
        const { food, newAmount } = event.detail;
        console.log(food, newAmount); 
        try {
            await invoke('update_food', { food, newAmount });
            await getMealFoods(); 
            await getMealMacros(); 
            dispatch('updateTotals'); 
        } catch(error) {
            console.log(error); 
        }
    }

    function deleteMeal() {
        dispatch('delete', meal); 
    }

    function resetSelected(value) {
        // set the other value to null
        if (value === "food") {
            selectedRecipe = null; 
        } else {
            selectedFood = null;
        }
    }

</script>

<Card.Root>
    <div class="flex w-full justify-between">
        <Button class="rounded-none" variant="ghost" size="icon" on:click={() => renameDialogOpen = true}>
            <PencilLine class="h-4 w-4"/>
        </Button>
        <!-- ! DELETE BUTTON -->
		<Button class="rounded-none" variant="ghost" size="icon" on:click={() => deleteMeal()}>
			<X class="h-4 w-4"/>
		</Button>
	</div>
    <Card.Header class="pt-0">
        <!-- Active and Constant Buttones -->
        <Card.Title class="capitalize pb-2">{meal.name}</Card.Title>
        <Card.Description>
            <!-- ! SWITCHES -->
            <div class="flex justify-between">
                <div class="flex items-center space-x-2">
                    <Switch includeInput={true} id="is-constant" checked={isConstant} onCheckedChange={updateConstantStatus}/>
                    <Label for="is-constant">Constant</Label>
                </div>
                <div class="flex items-center space-x-2">
                    <Switch id="is-active" checked={isActive} onCheckedChange={updateDisabledStatus}/>
                    <Label for="is-active">Active</Label>
                </div>
            </div>
        </Card.Description>
    </Card.Header>
    <Card.Content>
        <div class="flex flex-col justify-center space-y-2">
            <Button on:click={() => addFoodDialogOpen = true}>
                <Plus class="mr-2 h-4 w-4"/>
                Add Food or Recipe
            </Button>
            <!-- * DIALOG FOR ADDING NEW FOOD -->
            <Dialog.Root bind:open={addFoodDialogOpen}>
                <Dialog.Trigger/>
                <Dialog.Content>
                    <Dialog.Header>
                        <Dialog.Title>Add Food or Recipe to <span class="capitalize">{meal.name}</span></Dialog.Title>
                        <Dialog.Description>Select one food or recipe and specify the amount</Dialog.Description>
                    </Dialog.Header>
                    <div class="flex flex-col justify-center items-center">
                        <RadioGroup.Root bind:value={selectionOption} onValueChange={resetSelected}>
                            <div class="flex items-center space-x-2">
                                <RadioGroup.Item value="food" id="foodOption"/>
                                <Label for="foodOption">Food</Label>
                            </div>
                            <div class="flex items-center space-x-2">
                                <RadioGroup.Item value="recipe" id="recipeOption"/>
                                <Label for="recipeOption">Recipe</Label>
                            </div>
                        </RadioGroup.Root>
                        <div class="grid grid-cols-4 items-center gap-4 mt-4">
                            <div class="col-span-2">
                            {#if selectionOption === "food"}
                                <Select.Root bind:selected={selectedFood}>
                                    <Select.Trigger>
                                        <Select.Value placeholder="Food"/>
                                    </Select.Trigger>
                                    <Select.Content>
                                        {#each $foodsNormalized as food}
                                        <Select.Item value={food} label={food.name} class="capitalize"/>
                                        {/each}
                                    </Select.Content>
                                </Select.Root>
                            {:else}
                                <Select.Root bind:selected={selectedRecipe}>
                                    <Select.Trigger>
                                        <Select.Value placeholder="Recipe"/>
                                    </Select.Trigger>
                                    <Select.Content>
                                        {#each $recipes as recipe}
                                        <Select.Item value={recipe} label={recipe.name} class="capitalize"/>
                                        {/each}
                                    </Select.Content>
                                </Select.Root>
                            {/if}
                        </div>
                        <!-- Amount input box -->
                        <Label for="amount" class="text-right">Amount<br/>{selectedFood ? `(${selectedFood.value.unit})` : (selectedRecipe ? `(${selectedRecipe.value.unit})` : "")}</Label>
                            <Input id="amount" bind:value={selectedAmount} type="number" />
                        </div>
                    </div>
                    <Dialog.Footer>
                        <Button disabled={isNaN(selectedAmount) || !selectedAmount || selectedAmount < 0} on:click={addNewFoodToMeal}>Confirm</Button>
                        <Dialog.Close>
                            <Button on:click={() => {selectedFood = null; selectedRecipe = null; selectedAmount = null;}}>Cancel</Button>
                        </Dialog.Close>
                    </Dialog.Footer>
                </Dialog.Content>
            </Dialog.Root>
            <!-- * DIALOG FOR RENAMING -->
            <Dialog.Root bind:open={renameDialogOpen}>
                <Dialog.Trigger/> 
                <Dialog.Content>
                    <Dialog.Header>
                        <Dialog.Title>Rename Meal</Dialog.Title>
                    </Dialog.Header>
                    <div class="grid grid-cols-4 items-center gap-4 mt-4">
                        <Label class="text-right col-span-1">New Name</Label>
                        <Input bind:value={newName} type="text" class="col-span-3" />
                    </div>
                    <Dialog.Footer>
                        <Button on:click={() => handleRename()}>Confirm</Button>
                        <Dialog.Close>
                            <!-- Reset the name -->
                            <Button on:click={() => newName = meal.name}>Cancel</Button>
                        </Dialog.Close>
                    </Dialog.Footer>
                </Dialog.Content>
            </Dialog.Root>
            <!-- ! LOOP -->
            <div class="grid grid-cols-1 gap-2">
            {#each mealFoods as food}
                <SingleMealFood {food} on:delete={deleteFoodFromMeal} on:update={editFoodInMeal}/>
            {/each}
            </div>
        </div>
        <!-- ! TABLE WITH MACROS -->
        {#if Object.keys(mealMacros).length !== 0}
        <div class="pt-4">
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
    </div>

        {/if}
    </Card.Content>
</Card.Root>