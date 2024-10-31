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
	{#if user.failed_time > 0}
		<h1>{user.username}</h1>
		{#if user.failed}
			<p>{failTimeToString(user.failed_time)}</p>
			<p>This is what you had to say: {user.failed_msg}</p>
		{:else}
			<input type="text" placeholder="It was too hard :((" bind:value={failMessage}>
			<button on:click={() => fail(failMessage).then((newStatus) => user = {...user, ...newStatus})}>
				I failed
			</button>
		{/if}
	{/if}
</div>