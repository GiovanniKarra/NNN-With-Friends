<script>
// @ts-nocheck

	import NavButton from "./NavButton.svelte";

	let tabs = ["Home", "Login", ["Groups", ["Group1", "Group2"]]]
</script>

<div class="navbar">
	{#each tabs as tab}
		{#if typeof(tab) === "string"}
			<NavButton name={tab} state={[tab.toLowerCase()]}/>
		{:else}
			<NavButton name={tab[0]} state={[tab[0].toLowerCase()]} dropdown={tab[1]}/>
		{/if}
	{/each}
</div>

<style>
	.navbar {
		display: flex;
		flex-direction: row;
		
		background-color: lightgray;
		margin: 0;
		padding: 0;
	}
</style>