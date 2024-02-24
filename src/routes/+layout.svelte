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
<header>
	<div class="text-sm font-medium text-center text-gray-500 border-b border-gray-200 ">
		<ul class="flex flex-wrap -mb-px justify-center">
			<li class="mr-2">
				<a href="/" on:click={() => selectPage('/')} class="inline-block p-4 border-b-2 border-transparent rounded-t-lg  {$currentPage === '/' ? 'text-purple-600 border-purple-600 active' : 'hover:text-gray-600 hover:border-gray-300'}">
					{$_('home.nav')}
				</a>
			</li>
			<li class="mr-2">
				<a href="/foods" on:click={() => selectPage('/foods')} class="inline-block p-4 border-b-2 border-transparent rounded-t-lg  {$currentPage === '/foods' ? 'text-purple-600 border-purple-600 active' : 'hover:text-gray-600 hover:border-gray-300'}">
					{$_('foods.nav')}
				</a>
			</li>
			<li class="mr-2">
				<a href="/recipes" on:click={() => selectPage('/recipes')} class="inline-block p-4 border-b-2 border-transparent rounded-t-lg  {$currentPage === '/recipes' ? 'text-purple-600 border-purple-600 active' : 'hover:text-gray-600 hover:border-gray-300'}">
					{$_('recipes.nav')}
				</a>
			</li>
			<li class="mr-2">
				<a href="/meals" on:click={() => selectPage('/meals')} class="inline-block p-4 border-b-2 border-transparent rounded-t-lg  {$currentPage === '/meals' ? 'text-purple-600 border-purple-600 active' : 'hover:text-gray-600 hover:border-gray-300'}">
					{$_('meals.nav')}
				</a>
			</li>
			<li class="mr-2">
				<a href="/logging" on:click={() => selectPage('/logging')}  class="inline-block p-4 border-b-2 border-transparent rounded-t-lg  {$currentPage === '/logging' ? 'text-purple-600 border-purple-600 active' : 'hover:text-gray-600 hover:border-gray-300'}">
					{$_('logs.nav')}
				</a>
			</li>
			<li class="mr-2">
				<a href="/graphs" on:click={() => selectPage('/graphs')}  class="inline-block p-4 border-b-2 border-transparent rounded-t-lg  {$currentPage === '/graphs' ? 'text-purple-600 border-purple-600 active' : 'hover:text-gray-600 hover:border-gray-300'}">
					{$_('charts.nav')}
				</a>
			</li>
			<div class="inline-flex my-2">
				<LanguageSelection />
				</div>
		</ul>
		
	</div>
</header>


<slot></slot>