<script>
    import { getGroupStatus } from "./api";
    import { pageState } from "./state";
    import UserInfoBox from "./UserInfoBox.svelte";

	let currentGroupID = "";
	let users = [];
	updateUsers();
	pageState.subscribe((state) => {
		currentGroupID = state.arg;
		updateUsers();
	});

	async function updateUsers() {
		users = currentGroupID === ""? []: await getGroupStatus(currentGroupID);
	}
</script>

<div class="group-users-display">
	{#if users.length > 0}
		{#each users as user}
			<UserInfoBox user={{...user, failed: false}}/>
		{/each}
	{/if}
</div>

<style>
	.group-users-display {
		display: flex;
		flex-wrap: wrap;
		column-gap: 10px;
		row-gap: 10px;
	}
</style>