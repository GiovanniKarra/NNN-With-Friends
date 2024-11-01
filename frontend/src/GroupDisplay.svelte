<script>
    import { getGroupStatus } from "./api";
    import { pageState } from "./state";
    import UserInfoBox from "./UserInfoBox.svelte";

	$: currentGroupID = $pageState.arg;
</script>

<div class="group-users-display">
	{#await getGroupStatus(currentGroupID) then users}
		{#each users as user}
			<UserInfoBox user={{...user, failed: false}}/>
		{/each}
	{/await}
</div>

<style>
	.group-users-display {
		display: flex;
		flex-wrap: wrap;
	}
</style>