<script>
	export let name = "";
	export let dropdown = [];

	let hover = false;
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<!-- svelte-ignore a11y-mouse-events-have-key-events -->
<div class="navbutton" on:mouseover={() => hover = true} on:mouseleave={() => hover = false}>
	<button on:click={() => fetch("/yo")}>
		{name}
	</button>

	{#if hover}
		{#each dropdown as drop}
			<button>
				{drop}
			</button>
		{/each}
	{/if}
</div>

<style>
	.navbutton {
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}
</style>