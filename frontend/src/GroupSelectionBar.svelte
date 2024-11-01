<script>
    import { createGroup, getMyGroups, joinGroup, leaveGroup } from "./api";
    import CreateGroupInput from "./CreateGroupInput.svelte";
    import GroupButton from "./GroupButton.svelte";
    import { pageState } from "./state";

	$: currentGroupID = $pageState.arg;
	let creatingGroup = false;
	let newGroupName = "";
	let errorMsg = "";

	let groups = [];
	updateGroups();
	$: {
		if (groups.length > 0) {
			let unknownGroup = true;
			groups.forEach((group) => {
				if (group.id == currentGroupID) {
					unknownGroup = false;
				}
			});
			if (unknownGroup && currentGroupID !== "") {
				joinGroup(currentGroupID);
				updateGroups();
			}
		}
	}

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
		<CreateGroupInput newGroup={newGroup}/>
	{:else}
		<button on:click={() => creatingGroup = true}>
			+ Create
		</button>
	{/if}
	{#each groups as group}
		<GroupButton group={group}/>
	{/each}
</div>

<style>
	.group-selection-bar {
		width: 20%;
		/* background-color: gray; */
	}
	button {
		width: 100%;
		margin: 0;
		border-radius: 0;
		border-width: 0;
	}
</style>