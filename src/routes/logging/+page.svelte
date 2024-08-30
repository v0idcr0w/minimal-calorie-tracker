<script>
    import Logs from "../../lib/LogsComponents/Logs.svelte";
    import { _ } from 'svelte-i18n'; 
    import { Separator } from '$lib/components/ui/separator';
</script>

<div class="space-y-4 m-4">
    <div>
        <h3 class="text-3xl font-medium">{$_('logs.title')}</h3>
        <p class="text-muted-foreground text-sm">
            See your progress in a tabular format. 
        </p>
    </div>
    <Separator />
    
    <Logs />
</div>

