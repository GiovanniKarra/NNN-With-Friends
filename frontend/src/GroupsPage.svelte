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
	<p>Share the group's link to a friend to add them!</p>
	<div class="vertical-split">
		<GroupSelectionBar/>
		<GroupDisplay/>
	</div>
</div>

<style>
	.groups-page {
		text-align: center;
	}
	.vertical-split {
		display: flex;
		flex-direction: row;
	}
</style>