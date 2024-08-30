<script>
    import Meals from "../../lib/MealsComponents/Meals.svelte";
    import { _ } from 'svelte-i18n'; 
    import { Separator } from '$lib/components/ui/separator';

</script>


<div class="space-y-4 m-4">
    <div>
        <h3 class="text-3xl font-medium">{$_('meals.title')}</h3>
        <p class="text-muted-foreground text-sm">
            Add meals and track your daily intake.
        </p>
    </div>
    <Separator />
    
    <Meals />
</div>

