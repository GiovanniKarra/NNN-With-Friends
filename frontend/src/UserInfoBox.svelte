<script>
	export let user = {
		username: "",
		failed: false,
		failed_time: 0,
		failed_msg: ""
	};
	$: failDate = new Date(user.failed_time);
	$: user.failed = user.failed_time > 0;
</script>

<div class={user.failed? "failed": "user-info-box"}>
	<p>{user.username}</p>
	{#if user.failed}
		<p>failed {failDate.toUTCString()}</p>
		<p>{user.failed_msg}</p>
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
		background-color: green;
	}
	.failed {
		border: 1px solid;
		border-radius: 10px;
		padding: 10px;
		margin: 20px;
		height: 200px;
		width: 200px;
		background-color: red;
	}
</style>