<script>
	export let user = {
		username: "",
		failed: false,
		fail_time: 0,
		fail_msg: ""
	};
</script>

<div class="user-info-box">
	<p>{user.username}</p>
	<p>{user.failed}</p>
	<p>{user.fail_time}</p>
	<p>{user.fail_msg}</p>
</div>

<style>
	.user-info-box {
		border: 1px solid;
		border-radius: 10px;
		padding: 10px;
		margin: 20px;
		height: 200px;
		width: 200px;
	}
</style>