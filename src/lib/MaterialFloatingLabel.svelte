<script>
    export let value; 
    export let label; 
    export let type; 
</script>

{#if type === 'number'}
<div class="relative z-0 mt-1 mb-3">
    <input id="floating_standard" class="block py-2.5 px-0 w-full text-sm text-neutral-600 bg-transparent border-0 border-b-2 border-gray-300 appearance-none focus:outline-none focus:ring-0 focus:border-purple-600 peer" type="number"  placeholder='' bind:value={value} />
    <label for="floating_standard" class="absolute text-sm text-gray-500 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:start-0 peer-focus:text-purple-600 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto">{label}</label>
</div>

{:else}
<div class="relative z-0 mt-1 mb-3">
    <input id="floating_standard" class="block py-2.5 px-0 w-full text-sm text-neutral-600 bg-transparent border-0 border-b-2 border-gray-300 appearance-none focus:outline-none focus:ring-0 focus:border-purple-600 peer" placeholder="" bind:value={value} />
    <label for="floating_standard" class="absolute text-sm text-gray-500 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:start-0 peer-focus:text-purple-600 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto">{label}</label>
</div>
{/if}