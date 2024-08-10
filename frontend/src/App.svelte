<script>
	import NavBar from "./NavBar.svelte";
	import LoginPage from "./LoginPage.svelte";
	import { pageState } from "./stores";


	const stateToComponent = {
		"login": LoginPage
	}
	let currentPage = "";

	pageState.subscribe(
		(value) => {
			window.history.replaceState(value, "", "/"+value.path.join("/"));
			currentPage = value.path[0];
		}
	);

</script>

<header>
	<title>NNN With Friends</title>
</header>
<main>
	<NavBar />
	<svelte:component this={stateToComponent[currentPage]} />
</main>

<style>
	
</style>
