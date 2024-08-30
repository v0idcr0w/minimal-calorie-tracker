<script>
	import '../styles.css'; 
	import { writable } from 'svelte/store';
	import { init, getLocaleFromNavigator, locale, dictionary, _ } from 'svelte-i18n';
	import LanguageSelection from '../lib/LanguageSelection.svelte'; 

	import en from '../lang/en.json';
	import pt_br from '../lang/pt_br.json';

	init({
	fallbackLocale: 'en',
	initialLocale: getLocaleFromNavigator(),
	});
	
	dictionary.set({ en, pt_br });

	const currentPage = writable(''); 
	const selectPage = (page) => {
		currentPage.set(page); 
	}
</script>

<header class="border-border/40 bg-background/95 supports-[backdrop-filter]:bg-background/60 sticky top-0 z-50 w-full border-b backdrop-blur">
	<nav class="hidden flex-col gap-6 text-lg font-medium md:flex md:flex-row md:items-center md:gap-5 md:text-sm lg:gap-6">
		<a href="##" class="flex items-center gap-2 text-lg font-semibold md:text-base">
			<span class="sr-only"></span>
		  </a>
		  <a href="/" on:click={() => selectPage('/')} class="{$currentPage === '/' ? 'text-foreground': 'text-muted-foreground'} hover:text-foreground transition-colors">
			{$_('home.nav')}
		  </a>
		  <a href="/foods" on:click={() => selectPage('/foods')} class="{$currentPage === '/foods' ? 'text-foreground': 'text-muted-foreground'} hover:text-foreground transition-colors">
			{$_('foods.nav')}
		  </a>
		  <a href="/recipes" on:click={() => selectPage('/recipes')} class="{$currentPage === '/recipes' ? 'text-foreground': 'text-muted-foreground'} hover:text-foreground transition-colors">
			{$_('recipes.nav')}
		  </a>
		  <a href="/meals" on:click={() => selectPage('/meals')} class="{$currentPage === '/meals' ? 'text-foreground': 'text-muted-foreground'} hover:text-foreground transition-colors">
			{$_('meals.nav')}
		  </a>
		  <a href="/meal_history" on:click={() => selectPage('/meal_history')} class="{$currentPage === '/meal_history' ? 'text-foreground': 'text-muted-foreground'} hover:text-foreground transition-colors">
			{$_('meal_history.nav')}
		  </a>
		  <a href="/logging" on:click={() => selectPage('/logging')} class="{$currentPage === '/logging' ? 'text-foreground': 'text-muted-foreground'} hover:text-foreground transition-colors">
			{$_('logs.nav')}
		  </a>
		  <a href="/graphs" on:click={() => selectPage('/graphs')} class="{$currentPage === '/graphs' ? 'text-foreground': 'text-muted-foreground'} hover:text-foreground transition-colors">
			{$_('charts.nav')}
		  </a>
		  <div class="inline-flex my-2">
			<LanguageSelection />
		  </div>
	</nav>
</header>

<!-- ! BODY -->
<slot></slot>