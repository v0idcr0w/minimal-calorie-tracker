<script>
	import '../styles.css'; 
	import { writable } from 'svelte/store';
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
					Home
				</a>
			</li>
			<li class="mr-2">
				<a href="/foods" on:click={() => selectPage('/foods')} class="inline-block p-4 border-b-2 border-transparent rounded-t-lg  {$currentPage === '/foods' ? 'text-purple-600 border-purple-600 active' : 'hover:text-gray-600 hover:border-gray-300'}">
					Foods
				</a>
			</li>
			<li class="mr-2">
				<a href="/recipes" on:click={() => selectPage('/recipes')} class="inline-block p-4 border-b-2 border-transparent rounded-t-lg  {$currentPage === '/recipes' ? 'text-purple-600 border-purple-600 active' : 'hover:text-gray-600 hover:border-gray-300'}">
					Recipes
				</a>
			</li>
			<li class="mr-2">
				<a href="/meals" on:click={() => selectPage('/meals')} class="inline-block p-4 border-b-2 border-transparent rounded-t-lg  {$currentPage === '/meals' ? 'text-purple-600 border-purple-600 active' : 'hover:text-gray-600 hover:border-gray-300'}">
					Meals
				</a>
			</li>
			<li class="mr-2">
				<a href="/logging" on:click={() => selectPage('/logging')}  class="inline-block p-4 border-b-2 border-transparent rounded-t-lg  {$currentPage === '/logging' ? 'text-purple-600 border-purple-600 active' : 'hover:text-gray-600 hover:border-gray-300'}">
					Daily Logs
				</a>
			</li>
			<li class="mr-2">
				<a href="/graphs" on:click={() => selectPage('/graphs')}  class="inline-block p-4 border-b-2 border-transparent rounded-t-lg  {$currentPage === '/graphs' ? 'text-purple-600 border-purple-600 active' : 'hover:text-gray-600 hover:border-gray-300'}">
					Progress Plots
				</a>
			</li>
		</ul>
	</div>
</header>


<slot></slot>