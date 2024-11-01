<script>
    import { createGroup, getMyGroups, joinGroup, leaveGroup } from "./api";
    import { pressedEnter } from "./misc";
    import { pageState } from "./state";

	export let currentGroupID = "";
	let creatingGroup = false;
	let newGroupName = "";
	let errorMsg = "";

	let groups = [];
	updateGroups();

	function newGroup() {
		createGroup(newGroupName).then((result) => {
			if (!result.success) errorMsg = result.groupid;
			else joinGroup(result.groupid).then(() => {
				creatingGroup = false;
				updateGroups();
			});
		})
	}
	async function updateGroups() {
		groups = await getMyGroups();
	}
</script>


<div class="group-selection-bar">
	{#if creatingGroup}
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
	{:else}
		<button on:click={() => creatingGroup = true}>
			+ Create
		</button>
	{/if}
	{#each groups as group}
		<div class="duo">
			<button
				style="background-color: red; width: 25%;"
				on:click={() => {
					let yes = confirm(`Are you sure you want to quit ${group.name}?`);
					if (!yes) return;
					if (currentGroupID == group.id)
						pageState.update((current) => ({...current, arg:""}))
					leaveGroup(group.id);
				}}>
				X
			</button>
			<button
				on:click={() => pageState.update((current) => ({...current, arg: group.id}))}
				disabled="{currentGroupID == group.id}">
				{group.name} <br>
				<small>by {group.founder}</small>
			</button>
		</div>
	{/each}
</div>

<style>
	.group-selection-bar {
		width: 20%;
		/* background-color: gray; */
	}
	.duo {
		display: flex;
		flex-direction: row;
		height: auto;
	}
	button {
		width: 100%;
		margin: 0;
		border-radius: 0;
		border-width: 0;
	}
	input {
		border-color: black;
		margin: 0;
	}
</style>