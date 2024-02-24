<script>
import Logs from "../../lib/Logs.svelte";
import Tooltip from '../../lib/Tooltip.svelte'; 
import { _ } from 'svelte-i18n'; 
</script>
<h1 class="text-neutral-600 text-3xl m-4 font-bold tracking-loose">{$_('logs.title')}
<Tooltip text={$_('logs.help')} />
</h1>
<Logs />