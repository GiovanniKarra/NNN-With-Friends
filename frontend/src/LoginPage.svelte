<script>
    import { login, signup } from "./login";
    import { pressedEnter } from "./misc";

	let username = ""; let password = "";
	let createAccount = false;
	let errorMessage = "";

	function goButton() {
		(createAccount? signup(username, password).then(() => login(username, password)):
		login(username, password)).then((msg) => errorMessage = msg.toString())
	}
</script>

<div class="login-page">
	<h1>LOGIN</h1>
	<div class="login-form">
		<form>
			<label for="username">Username</label>
			<input type="text" placeholder="NNN-Enjoyer-69" required bind:value={username}
				on:keydown={(e) => pressedEnter(e, goButton)} maxlength="32"/>
			<label for="password">Password</label>
			<input type="password" required bind:value={password}
				on:keydown={(e) => pressedEnter(e, goButton)} maxlength="64"/>
			<input type="button" on:click={goButton}
				value="{createAccount? "Sign Up": "Log In"}">
			<input type="button" on:click={() => createAccount = !createAccount}
				value="{createAccount? "Log in to an existing account": "Create a new account"}">
			<p>{errorMessage}</p>
		</form>
	</div>
</div>

<style>
	.login-page {
		text-align: center;
		display: flex;
		align-items: center;
		flex-direction: column;
		width: 100%;
	}
	.login-form {
		width: 20%;
	}
	form {
		text-align: left;
		display: flex;
		flex-flow: column;
	}
</style>