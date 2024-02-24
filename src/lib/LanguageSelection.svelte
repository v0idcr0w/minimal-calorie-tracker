<script>
    import { locale, _ } from 'svelte-i18n'; 
    let dropdownActive = false; 
    const locDict = {'English': 'en', 'Português (BR)': 'pt_br'}; 
    function setLang(loc) {
        $locale = locDict[loc]; 
        dropdownActive = false; 
    }
</script>
<div class="relative inline-block text-left">
    <div>
      <button type="button" class="inline-flex w-full justify-center gap-x-1.5 rounded-md bg-white px-3 py-2 text-sm text-gray-500 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50" id="menu-button" aria-expanded="true" aria-haspopup="true" on:click={() => dropdownActive = !dropdownActive}>
        {$_('language')}
        <svg class="-mr-1 h-5 w-5 text-gray-500" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
          <path fill-rule="evenodd" d="M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z" clip-rule="evenodd" />
        </svg>
      </button>
    </div>

{#if dropdownActive}
    <div class="absolute right-0 z-10 mt-1 w-full origin-top-right rounded-md bg-white shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none" role="menu" aria-orientation="vertical" aria-labelledby="menu-button" tabindex="-1">
      <div class="py-1" role="none">
          {#each Object.keys(locDict) as loc}
            <button class="block w-full z-[1000] px-4 py-2 text-right text-sm font-normal text-gray-500 hover:bg-neutral-100 active:text-neutral-800 active:no-underline disabled:pointer-events-none disabled:bg-transparent disabled:text-neutral-400" role="menuitem" on:click={() => setLang(loc)}>  {loc}  </button>
          {/each}
      </div>
    </div>
{/if}
  </div>