<script>
	import { failTimeToString, successToString } from "./misc";
    import { pageState } from "./state";

	export let user = {
		username: "",
		failed: false,
		failed_time: 0,
		failed_msg: ""
	};
	$: user.failed = user.failed_time > 0;
</script>

<div class={user.failed? "failed": "user-info-box"}>
	<p><strong>{user.username}</strong></p>
	{#if user.failed}
		<p>{failTimeToString(user.failed_time)}</p>
		<p>"{user.failed_msg}"</p>
	{:else}
		<p>{successToString(Math.floor(Date.now()/1000) - $pageState.interval[0])}</p>
	{/if}
</div>

<style>
	.user-info-box {
		border: 1px solid;
		border-radius: 10px;
		padding: 10px;
		margin: 20px;
		height: 200px;
		width: 200px;
		background-color: var(--color-table);
		text-align: center;
	}
	.failed {
		border: 1px solid;
		border-radius: 10px;
		padding: 10px;
		margin: 20px;
		height: 200px;
		width: 200px;
		background-color: red;
		text-align: center;
	}
</style>