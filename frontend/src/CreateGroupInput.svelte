<script>
    import { pressedEnter } from "./misc";

	export let newGroup;
	let newGroupName = "";
	let errorMsg = "";
</script>

<div class="duo">
	<input type="text" bind:value={newGroupName}
		on:keydown={(event) => pressedEnter(event, newGroup)}>
	<button on:click={newGroup} style="width: 20%; padding-left: 16px;">
		New
	</button>
</div>
{#if errorMsg.length > 0}
	<p>{errorMsg}</p>
{/if}

<style>
	.duo {
		display: flex;
		flex-direction: row;
		height: auto;
	}
	input {
		border-color: black;
		margin: 0;
	}
	button {
		width: 100%;
		margin: 0;
		border-width: 0;
	}
</style>