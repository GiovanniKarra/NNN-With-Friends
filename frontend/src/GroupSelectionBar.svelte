<script>
    import { getMyGroups } from "./api";
    import { pageState } from "./state";

	export let currentGroupID = "";
</script>


<div class="group-selection-bar">
	{#await getMyGroups() then groups}
		{#each groups as group}
			<button
				on:click={() => pageState.update((current) => ({...current, arg: group.id}))}
				disabled="{currentGroupID == group.id}">
				{group.name} <br>
				<small>by {group.founder}</small>
			</button>
		{/each}
	{/await}
</div>

<style>
	.group-selection-bar {
		width: 20%;
		background-color: gray;
	}	
</style>