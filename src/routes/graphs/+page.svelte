<script>
    import ProgressChart from "../../lib/ChartComponents/ProgressChart.svelte";
    import { _ } from 'svelte-i18n'; 
    import { Separator } from '$lib/components/ui/separator';
</script>

<div class="space-y-4 m-4">
    <div>
        <h3 class="text-3xl font-medium">{$_('charts.title')}</h3>
        <p class="text-muted-foreground text-sm">
            See your progress as a chart.
        </p>
    </div>
    <Separator />
    
    <ProgressChart />
</div>
