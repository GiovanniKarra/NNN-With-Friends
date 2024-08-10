<script>
	let username; let password;
	async function signup() {
		let ret = await fetch(`/api/signup`, { 
			method: "POST",
			body: JSON.stringify({ username: username, password: password}) }
		)
		console.log(ret)
	}
</script>


<div class="signup-form">
	<h4>Login</h4>
	<input type="text" name="username" id="username" placeholder="username" bind:value={username}>
	<input type="password" name="password" id="password" placeholder="password" bind:value={password}>
	<button class="Submit" on:click={signup}>Signup</button>
</div>