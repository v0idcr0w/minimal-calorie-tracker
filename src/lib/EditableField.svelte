<script>
    export let text;
    export let handleRename; 
    export let obj; 
    let div;

    function makeEditable() {
        div.contentEditable = 'true';
        div.focus();
    }

    async function handleBlur() {
        text = div.textContent;
        div.contentEditable = 'false';
        await handleRename(obj, text); 
    }
</script>

<h3 bind:this={div} on:dblclick={makeEditable} on:blur={handleBlur} class="text-neutral-700 text-xl m-4 font-bold">
    {text}
</h3>