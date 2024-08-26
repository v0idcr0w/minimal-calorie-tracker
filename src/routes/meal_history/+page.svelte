<script>
    import Tooltip from "../../lib/Tooltip.svelte";
    import SvgInfo from "../../lib/SvgInfo.svelte";
    import { _ } from 'svelte-i18n'; 
    import MealHistory from "../../lib/MealsComponents/MealHistory.svelte";
    
</script>
<h1 class="text-neutral-600 text-3xl m-4 font-bold tracking-loose">{$_('meal_history.title')}
    <Tooltip text={$_('meal_history.help')}> 
        <SvgInfo />
    </Tooltip>
</h1>
<MealHistory />
    