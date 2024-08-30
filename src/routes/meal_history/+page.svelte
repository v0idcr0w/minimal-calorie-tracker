<script>
    import { _ } from 'svelte-i18n'; 
    import MealHistory from "../../lib/MealsComponents/MealHistory.svelte";
    import { Separator } from '$lib/components/ui/separator';
    
</script>
<div class="space-y-4 m-4">
    <div>
        <h3 class="text-3xl font-medium">{$_('meal_history.title')}</h3>
        <p class="text-muted-foreground text-sm">
            View past meals and their nutritional information. Use the Constant toggle button to transfer the meal to the current day. 
        </p>
    </div>
    <Separator />
    
    <MealHistory />
</div>

    