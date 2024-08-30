<script>
    import Recipes from "$lib/RecipesComponents/Recipes.svelte";
    import { _ } from 'svelte-i18n'; 
    import { Separator } from '$lib/components/ui/separator';
</script>

<div class="space-y-4 m-4">
    <div>
        <h3 class="text-3xl font-medium">{$_('recipes.title')}</h3>
        <p class="text-muted-foreground text-sm">
            Add, view and edit your recipes, including their ingredients and nutritional information.
        </p>
    </div>
    <Separator />
    
    <Recipes />
</div>

