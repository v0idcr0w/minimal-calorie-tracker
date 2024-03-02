<script>
    import Recipes from "../../lib/Recipes.svelte";
    import Tooltip from '../../lib/Tooltip.svelte'; 
    import SvgInfo from "../../lib/SvgInfo.svelte";
    import { _ } from 'svelte-i18n'; 
	import SvgAdd from "../../lib/SvgAdd.svelte";
</script>
<h1 class="text-neutral-600 text-3xl m-4 font-bold tracking-loose">{$_('recipes.title')}
    <Tooltip text={$_('recipes.help')}>
        <SvgInfo />
    </Tooltip>
</h1>

<Recipes />
