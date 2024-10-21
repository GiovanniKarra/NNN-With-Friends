<script>
// @ts-nocheck

    import { pageState } from "./state";

	export let page = ""; export let arg = "";
	$: name = page.charAt(0).toUpperCase() + page.slice(1);
</script>


<div class="navbutton">
	<button on:click={pageState.update((current) =>
			({...current, "page": page, "arg": arg}))}>
		{name}
	</button>
</div>