<script>
    import Home from '../lib/HomeComponents/Home.svelte'; 
    import { _ } from 'svelte-i18n'; 

    import { Separator } from '$lib/components/ui/separator';

</script>
<div class="space-y-4 m-4">
<div>
    <h3 class="text-3xl font-medium">{$_('home.title')}</h3>
    <p class="text-muted-foreground text-sm">
        Track your daily weight and update your current goals.
    </p>
</div>
<Separator />
<Home />
</div>
