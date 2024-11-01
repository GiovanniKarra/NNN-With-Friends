<script>
    import { getMyGroups, joinGroup } from "./api";
    import CreateGroupInput from "./CreateGroupInput.svelte";
    import GroupButton from "./GroupButton.svelte";
    import { pageState } from "./state";

	$: currentGroupID = $pageState.arg;

	let groups = [];
	let groupsUpdated = false;
	updateGroups();
	$: {
		if (groupsUpdated) {
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

	async function updateGroups() {
		groups = await getMyGroups();
		groupsUpdated = true;
	}
	pageState.subscribe((_) => updateGroups());
</script>


<div class="group-selection-bar">
	<CreateGroupInput updateGroups={updateGroups}/>
	{#each groups as group}
		<GroupButton group={group}/>
	{/each}
</div>

<style>
	.group-selection-bar {
		width: 20%;
		/* background-color: gray; */
	}
</style>