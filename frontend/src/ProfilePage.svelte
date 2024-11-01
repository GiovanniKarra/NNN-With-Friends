<script>
	import { fail, getStatus } from "./api";
    import { failTimeToString, pressedEnter } from "./misc";
	import { pageState } from "./state";

	let user = {
		username: "",
		failed: false,
		failed_time: 0,
		failed_msg: ""
	};
	let failMessage = "";
	pageState.subscribe((state) => {
		getStatus(state.user).then((u) => {
			user = {...user, ...u};
			user.username = state.user;
		});
	});
	function sendFail() {
		fail(failMessage).then((newStatus) => user = {...user, ...newStatus});
	}
</script>

<div class="profile-page">
	{#if user.username.length > 0}
		<h1>{user.username}</h1>
		{#if user.failed}
			<p>{failTimeToString(user.failed_time)}</p>
			<p>This is what you had to say: {user.failed_msg}</p>
		{:else}
			<div class="fail-input">
				<input type="text" placeholder="It was too hard :((" bind:value={failMessage}
					style="margin:0;" maxlength="80"
					on:keydown={(e) => pressedEnter(e, sendFail)}>
				<button on:click={sendFail} style="margin:0;">
					I failed
				</button>
			</div>
		{/if}
	{/if}
</div>

<style>
	.profile-page {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		text-align: center;
		width: 100%;
	}
	.fail-input {
		display: flex;
		flex-direction: row;
		width: 100%;
		padding-left: 35%;
	}
</style>