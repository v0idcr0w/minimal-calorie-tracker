<script>
    import Home from '../lib/HomeComponents/Home.svelte'; 
    import { _ } from 'svelte-i18n'; 
</script>
<h1 class="text-neutral-600 text-3xl m-4 font-bold tracking-loose">
    {$_('home.title')}
</h1>
<Home />