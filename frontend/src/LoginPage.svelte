<script>
	import CreateForm from "./CreateForm.svelte";
	import LoginForm from "./LoginForm.svelte";

	let create = false;
</script>

{#if create}
	<CreateForm />
{:else}
	<LoginForm />
{/if}

<button class="toggle-mode" on:click={() => create = !create}>
	{create? "go to login page": "create account"}
</button>


<style>
	.toggle-mode {
		background: none;
		border: none;
		text-decoration: underline;
		color: blue;
	}
	.toggle-mode:hover {
		text-decoration: none;
		cursor: pointer;
	}
</style>
