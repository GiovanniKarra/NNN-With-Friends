<script>
    import { createGroup, getMyGroups, joinGroup, leaveGroup } from "./api";
    import { pageState } from "./state";

	export let currentGroupID = "";
	let creatingGroup = false;
	let newGroupName = "";
	let errorMsg = "";
</script>


<div class="group-selection-bar">
	{#if creatingGroup}
		<input type="text" bind:value={newGroupName}>
		<button on:click={() => {
			createGroup(newGroupName).then((result) => {
				if (!result.success) errorMsg = result.groupid;
				else joinGroup(result.groupid).then(() => creatingGroup = false);
			})
		}}>
			go
		</button>
		<p>{errorMsg}</p>
	{:else}
		<button on:click={() => creatingGroup = true}>
			+ Create
		</button>
	{/if}
	{#key [creatingGroup, currentGroupID]}
		{#await getMyGroups() then groups}
			{#each groups as group}
				<button
					on:click={() => pageState.update((current) => ({...current, arg: group.id}))}
					disabled="{currentGroupID == group.id}">
					{group.name} <br>
					<small>by {group.founder}</small>
				</button>
				<button
					style="background-color: red;"
					on:click={() => {
						let yes = confirm(`Are you sure you want to quit ${group.name}?`);
						if (!yes) return;
						if (currentGroupID == group.id)
							pageState.update((current) => ({...current, arg:""}))
						leaveGroup(group.id);
					}}>
					X
				</button>
			{/each}
		{/await}
	{/key}
</div>

<style>
	.group-selection-bar {
		width: 20%;
		background-color: gray;
	}
	button {
		width: 100%;
	}
</style>