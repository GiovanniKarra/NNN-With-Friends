<script>
	import { fail, getStatus } from "./api";
    import { failTimeToString } from "./misc";
	import { pageState } from "./state";

	let user = {
		username: "",
		failed: false,
		failed_time: 0,
		failed_msg: ""
	};
	let failMessage = "";
	pageState.subscribe((state) => {
		user.username = state.user;
		getStatus(user.username).then((u) => user = {...user, ...u});
	});
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
					style="margin:0;" maxlength="80">
				<button on:click={() =>
					fail(failMessage).then((newStatus) => user = {...user, ...newStatus})}
					style="margin:0;">
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
		width: 100%;
	}
	.fail-input {
		display: flex;
		flex-direction: row;
		width: 100%;
		padding-left: 35%;
	}
</style>