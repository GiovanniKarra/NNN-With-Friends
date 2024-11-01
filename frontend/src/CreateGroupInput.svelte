<script>
    import { createGroup, joinGroup } from "./api";
    import { pressedEnter } from "./misc";

	export let updateGroups;
	let newGroupName = "";
	let errorMsg = "";
	let creatingGroup = false;

	function newGroup() {
		createGroup(newGroupName).then((result) => {
			if (!result.success) errorMsg = result.groupid;
			else joinGroup(result.groupid).then(() => {
				creatingGroup = false;
				updateGroups();
			});
		})
	}
</script>


{#if creatingGroup}
	<div class="duo">
		<input type="text" bind:value={newGroupName}
			maxlength="30"
			on:keydown={(event) => pressedEnter(event, newGroup)}>
		<button on:click={newGroup} style="width: 20%; padding-left: 16px;">
			New
		</button>
	</div>
	{#if errorMsg.length > 0}
		<p>{errorMsg}</p>
	{/if}
{:else}
	<button on:click={() => creatingGroup = true} style="border-radius: 0;">
		+ Create
	</button>
{/if}

<style>
	.duo {
		display: flex;
		flex-direction: row;
		height: auto;
	}
	input {
		border-color: black;
		margin: 0;
	}
	button {
		width: 100%;
		margin: 0;
		border-width: 0;
	}
</style>