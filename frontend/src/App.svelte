<script>
	import { get_state_from_url, pageState } from "./state";
	import NavBar from "./NavBar.svelte";
	import HomePage from "./HomePage.svelte";
	import LoginPage from "./LoginPage.svelte";
	import Page404 from "./Page404.svelte";
	import About from "./About.svelte";
    import { login } from "./login";
    import GroupsPage from "./GroupsPage.svelte";
    import ProfilePage from "./ProfilePage.svelte";
	
	login("", "");

	let tabs = {
		"home": HomePage,
		"groups": GroupsPage,
		"profile": ProfilePage,
		"about": About
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
	{#if $pageState.user === ""}
		<LoginPage/>
	{:else}
		<NavBar tabs={Object.keys(tabs)}/>
		{#key currentTab}{#key currentArg}
		<svelte:component this={tabs[currentTab] || Page404} arg={currentArg}/>
		{/key}{/key}
	{/if}
</main>

<style>
	
</style>
