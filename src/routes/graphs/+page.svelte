<script>
    import ProgressChart from "../../lib/ProgressChart.svelte";
    import Tooltip from '../../lib/Tooltip.svelte'; 
    import { _ } from 'svelte-i18n'; 
</script>

<h1 class="text-neutral-600 text-3xl m-4 font-bold tracking-loose">{$_('charts.title')}
<Tooltip text={$_('charts.help')} />
</h1>
<ProgressChart />