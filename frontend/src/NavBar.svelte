<script>
	import NavButton from "./NavButton.svelte";
	export let tabs = [];

</script>

<div class="navbar">
	{#each tabs as tab}
		<NavButton page={tab}/>
	{/each}
</div>

<style>
	.navbar {
		display: flex;
		flex-direction: row;
	}
</style>

