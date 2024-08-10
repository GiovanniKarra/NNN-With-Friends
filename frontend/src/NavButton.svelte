<script>
	import NavButton from "./NavButton.svelte";
	import { pageState } from "./stores";

	export let name = "";
	export let dropdown = [];
	export let state = [];

	let hover = false;

	let selected = false;
	pageState.subscribe(
		(value) => selected = state.join("/") === value.path.join("/")
	)
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<!-- svelte-ignore a11y-mouse-events-have-key-events -->
<div class="navbutton" on:mouseover={() => hover = true} on:mouseleave={() => hover = false}>
	<button style={selected? "background: grey": ""}
		on:click={() => pageState.update((current) => ({...current, path:state}))}>
		{name}
	</button>

	{#if hover}
		{#each dropdown as drop}
			<NavButton name={drop} state={state.concat(drop.toLowerCase())}/>
		{/each}
	{/if}
</div>

<style>
	.navbutton {
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}
	.navbutton > button {
		background: none;
		border: none;
		margin: 0;
		padding: 10px;
		cursor: pointer;
		font-size: 20pt;
	}
	.navbutton > button:hover {
		background: lightblue;
		transition-duration: 0.1s;
	}
</style>