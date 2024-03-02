<script>
	let isHovered = false; 
	export let text; 

	function onHover() {
		isHovered = true; 
	}
	function onLeave() {
		isHovered = false; 
	}
</script> 

<div role="tooltip" on:mouseover={onHover} on:focus={onHover} on:mouseleave={onLeave} class="relative inline-flex transition-colors duration-200 ease-in-out text-neutral-400 hover:text-neutral-800" >
	<div class="relative flex items-center justify-center">
		<slot></slot>

		<div class="absolute bg-neutral-900 text-white p-2 rounded-lg shadow-lg text-xs z-50 left-full bottom-full transition-opacity opacity-0 ease-in-out duration-200 {isHovered ? 'opacity-90' : 'opacity-0'} w-auto whitespace-pre tracking-loose {isHovered ? '' : 'invisible'}">
			{text}
		</div>
	</div>
</div>