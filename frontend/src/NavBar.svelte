<script>
    import { logout } from "./login";
	import NavButton from "./NavButton.svelte";
	export let tabs = [];
	
	function askLogout() {
		let yes = confirm(`Are you sure you want to logout?`);
		if (!yes) return;
		logout();
	}
</script>

<div class="navbar">
	{#each tabs as tab}
		<NavButton page={tab}/>
	{/each}
	<button style="border-radius: 0;" on:click={askLogout}>Logout</button>
</div>

<style>
	.navbar {
		display: flex;
		flex-direction: row;
		flex-wrap: wrap;
	}
</style>

