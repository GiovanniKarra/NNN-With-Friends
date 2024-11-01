<script>
	import { leaveGroup } from "./api";
	import { pageState } from "./state";

	export let group;
	$: currentGroupID = $pageState.arg;
</script>

<div class="group-button">
	<button
		style="background-color: red; width: 25%;"
		on:click={() => {
			let yes = confirm(`Are you sure you want to quit ${group.name}?`);
			if (!yes) return;
			leaveGroup(group.id).then(() => {
				pageState.update((current) =>
					({...current, arg:currentGroupID === group.id? "": current.arg}))
			});
		}}>
		X
	</button>
	<button
		on:click={() => pageState.update((current) => ({...current, arg: group.id}))}
		disabled="{currentGroupID === group.id}">
		{group.name} <br>
		<small>by {group.founder}</small>
	</button>
</div>

<style>
	.group-button {
		display: flex;
		flex-direction: row;
		height: auto;
	}
	button {
		width: 100%;
		margin: 0;
		border-radius: 0;
		border-width: 0;
		text-wrap: wrap;
	}
</style>