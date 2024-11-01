<script>
    import GroupDisplay from "./GroupDisplay.svelte";
	import GroupSelectionBar from "./GroupSelectionBar.svelte";
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
	@media (max-width: 500px) {
		.vertical-split {
			display: flex;
			flex-direction: row;
			flex-wrap: wrap;
		}
	}
</style>