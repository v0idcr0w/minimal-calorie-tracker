<script>
    export let error; 
    export let errorMessage; 
    export let value; 
</script>
<div class="relative z-0">
    <input type="text" id="standard_error" aria-describedby="standard_error_help" class="block py-2.5 px-0 w-full text-sm text-gray-900 bg-transparent border-0 border-b-2 border-red-600 appearance-none focus:outline-none focus:ring-0 focus:border-red-600 peer" placeholder=" " bind:value={value} />
    <label for="standard_error" class="absolute text-sm text-red-600  duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:start-0 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto">{error}</label>
</div>
<p id="standard_error_help" class="mt-2 text-xs text-red-600">{errorMessage}</p>