<script>
    import { getMyGroups, joinGroup } from "./api";
    import GroupDisplay from "./GroupDisplay.svelte";
	import GroupSelectionBar from "./GroupSelectionBar.svelte";
	import { pageState } from "./state";

	export let arg = $pageState.arg;

	$: {
		getMyGroups().then((groups) => {
			let unknownGroup = true;
			groups.forEach((group) => {
				if (group.id == arg) {
					unknownGroup = false;
				}
			});
			if (unknownGroup && arg !== "") joinGroup(arg);
		})
	}
</script>


<div class="groups-page">
	<h1>GROUPS</h1>
	<div class="vertical-split">
		<GroupSelectionBar currentGroupID={arg}/>
		<GroupDisplay currentGroupID={arg}/>
	</div>
</div>

<style>
	.vertical-split {
		display: flex;
		flex-direction: row;
	}
</style>