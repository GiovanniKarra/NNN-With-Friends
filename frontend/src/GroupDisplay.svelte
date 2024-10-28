<script>
    import { getGroupStatus } from "./api";
    import UserInfoBox from "./UserInfoBox.svelte";

	export let currentGroupID = "";
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