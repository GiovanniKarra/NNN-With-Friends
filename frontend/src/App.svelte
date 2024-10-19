<script>
	import { login } from "./login";
	import { get_state_from_url, pageState } from "./state";
    import NavBar from "./NavBar.svelte";
	import HomePage from "./HomePage.svelte";
    import LoginPage from "./LoginPage.svelte";

	let tabs = {
		"home": HomePage,
		"groups": LoginPage,
		"about": LoginPage
	}

	let currentTab = get_state_from_url().page
	let currentArg = get_state_from_url().arg

	pageState.subscribe((newState) => {
		currentTab = newState.page;
		currentArg = newState.arg;
	})
</script>

<header>
	<title>NNN With Friends</title>
</header>
<main>
	<NavBar/>
	{#key currentTab}{#key currentArg}
		<svelte:component this={tabs[currentTab]} arg={currentArg}/>
	{/key}{/key}
</main>

<style>
	
</style>
