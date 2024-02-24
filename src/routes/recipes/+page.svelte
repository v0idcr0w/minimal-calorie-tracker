<script>
    import Recipes from "../../lib/Recipes.svelte";
    import Tooltip from '../../lib/Tooltip.svelte'; 
    import { _ } from 'svelte-i18n'; 
</script>
<h1 class="text-neutral-600 text-3xl m-4 font-bold tracking-loose">{$_('recipes.title')}
    <Tooltip text={$_('recipes.help')} />
</h1>

<Recipes />
