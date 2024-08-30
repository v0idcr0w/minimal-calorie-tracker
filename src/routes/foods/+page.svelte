<script>
    import Foods from '../../lib/FoodsComponents/Foods.svelte'; 
    import { _ } from 'svelte-i18n'; 
    import { Separator } from '$lib/components/ui/separator';
</script>


<div class="space-y-4 m-4">
    <div>
        <h3 class="text-3xl font-medium">{$_('foods.title')}</h3>
        <p class="text-muted-foreground text-sm">
            Add, view and edit your foods and their nutritional information.
        </p>
    </div>
    <Separator />
    
    <Foods />
</div>
