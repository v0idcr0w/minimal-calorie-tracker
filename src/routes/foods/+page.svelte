<script>
    import Foods from '../../lib/Foods.svelte'; 
    import Tooltip from '../../lib/Tooltip.svelte'; 
    import SvgInfo from "../../lib/SvgInfo.svelte";
    import { _ } from 'svelte-i18n'; 
</script>

<h1 class="text-neutral-600 text-3xl m-4 font-bold tracking-loose">{$_('foods.title')}
    <Tooltip text={$_('foods.help')} >
        <SvgInfo />
    </Tooltip>
</h1> 
<Foods />