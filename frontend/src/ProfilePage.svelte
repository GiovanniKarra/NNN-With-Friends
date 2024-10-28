<script>
    import { fail, getStatus } from "./api";
    import { pageState } from "./state";

	let user = {
		username: "",
		failed: false,
		failed_time: 0,
		failed_msg: ""
	};
	pageState.subscribe((state) => {
		user.username = state.user;
		getStatus(user.username).then((u) => user = {...user, ...u});
	});
	let failMessage = "";
</script>

<div class="profile-page">
	<h1>{user.username}</h1>
	{#if user.failed}
		<p>Failed on {new Date(user.failed_time).toUTCString()}</p>
		<p>This is what they had to say: {user.failed_msg}</p>
	{:else}
		<input type="text" placeholder="It was too hard :((" bind:value={failMessage}>
		<button on:click={() => fail(failMessage).then((newStatus) => user = {...user, ...newStatus})}>
			I failed
		</button>
	{/if}
</div>