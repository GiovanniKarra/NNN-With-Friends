<script>
    import { createGroup, getMyGroups, joinGroup, leaveGroup } from "./api";
    import CreateGroupInput from "./CreateGroupInput.svelte";
    import GroupButton from "./GroupButton.svelte";
    import { pressedEnter } from "./misc";
    import { pageState } from "./state";

	$: currentGroupID = $pageState.arg;
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