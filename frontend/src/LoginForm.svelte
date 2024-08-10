<script>
	let username; let password;
	async function login() {
		let ret = username === "LOL"?
		await fetch(`/api/login`, { 
			method: "POST",
			headers: {"content-type": "application/json"} })
		: await fetch(`/api/login`, { 
			method: "POST",
			body: JSON.stringify({ username: username, password: password}),
			headers: {"content-type": "application/json"} }
		)
		console.log(ret)
	}
</script>


<div class="login-form">
	<h4>Login</h4>
	<input type="text" name="username" id="username" placeholder="username" bind:value={username}>
	<input type="password" name="password" id="password" placeholder="password" bind:value={password}>
	<button class="Submit" on:click={login}>Login</button>
</div>