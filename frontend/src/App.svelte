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
	import { getTimeInterval } from "./api";

	let tabs = {
		"home": HomePage,
		"groups": GroupsPage,
		"profile": ProfilePage,
		"about": About
	}

	let currentTab = get_state_from_url().page
	let currentArg = get_state_from_url().arg

	pageState.subscribe((newState) => {
		if (currentTab !== newState.page) currentTab = newState.page;
		currentArg = newState.arg;
	})

	async function preps() {
		await login("", "");
		let ret = await getTimeInterval();
		pageState.update((current) => ({...current, interval: ret}));
		return ret;
	}
</script>

<head>
	<title>NNN With Friends</title>
</head>
<main>
	{#await preps() then _}
		{#if $pageState.user === ""}
			<LoginPage/>
		{:else}
			<NavBar tabs={Object.keys(tabs)}/>
			<svelte:component this={tabs[currentTab] || Page404}/>
		{/if}
	{/await}
</main>

<style>
	main {
		padding: 0;
		margin: 0;
	}
</style>
